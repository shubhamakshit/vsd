use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{bail, Context, Result};
use kdam::term::Colorizer;

use crate::args::Quality;
use crate::utils::*;

pub struct DownloadState {
    args: crate::args::Args,
    downloader: crate::downloader::Downloader,
    audio_stream: bool,
    subtitle_stream: bool,
}

impl DownloadState {
    pub fn new() -> Result<Self> {
        let args = crate::args::parse();

        if args.capture {
            println!(
                "Launching chrome in headless={} mode for 3 minutes.",
                args.headless
            );
            crate::capture::run(args.input, args.headless);
            std::process::exit(0);
        }

        let downloader = crate::downloader::Downloader::new_custom(
            &args.user_agent,
            &args.header,
            &args.proxy_address,
        )
        .context("Couldn't create reqwest client.")?;

        if crate::utils::find_ffmpeg_with_path().is_none() {
            println!(
                "{} couldn't be located. Visit https://www.ffmpeg.org/download.html to install it.",
                "FFMPEG".colorize("bold red"),
            );
        }

        Ok(Self {
            args,
            downloader,
            audio_stream: false,
            subtitle_stream: false,
        })
    }

    fn get_url(&self, uri: &str) -> Result<String> {
        if uri.starts_with("http") {
            Ok(uri.to_owned())
        } else {
            if let Some(baseurl) = &self.args.baseurl {
                Ok(reqwest::Url::parse(baseurl)?.join(&uri)?.to_string())
            } else {
                Ok(reqwest::Url::parse(&self.args.input)?
                    .join(&uri)?
                    .to_string())
            }
        }
    }

    fn is_scrapable(&mut self) -> bool {
        crate::utils::find_hls_dash_links(&self.args.input).len() == 0
    }

    fn scrape_website(&mut self) -> Result<()> {
        println!("Scraping website for HLS and Dash links.");

        let resp = self
            .downloader
            .get(&self.args.input)
            .context("Couldn't scrape website. Make sure you are connected to internet.")?;

        if resp.status() == reqwest::StatusCode::OK {
            let links = crate::utils::find_hls_dash_links(&resp.text()?);

            match links.len() {
                0 => bail!(
                    "No links found on website source. Consider using {} flag.",
                    "--capture".colorize("bold green")
                ),
                1 => {
                    self.args.input = links[0].clone();
                    println!("Found one link {}", &links[0]);
                }
                _ => {
                    let mut elinks = vec![];
                    for (i, link) in links.iter().enumerate() {
                        elinks.push(format!("{:#2}) {}", i + 1, link));
                    }
                    let index = select("Select one link:".to_string(), elinks)?;
                    self.args.input = links[index].clone();
                }
            }
        } else {
            bail!(
                "{} returned HTTP status code {}",
                self.args.input,
                resp.status()
            );
        }

        Ok(())
    }

    fn parse_master(&self, master: &m3u8_rs::MasterPlaylist) -> Result<String> {
        let mut streams = vec![];
        let mut res_band: HashMap<&str, (usize, usize)> = HashMap::new();

        for (i, variant) in master.variants.iter().enumerate() {
            let bandwidth = variant.bandwidth.parse::<usize>().context(format!(
                "Couldn't parse bandwidth of variant playlist at index {}.",
                i
            ))?;
            let band_fmt = format_bytes(bandwidth);

            if let Some(resolution) = &variant.resolution {
                let res_fmt = match resolution.as_str() {
                    "256x144" => "144p",
                    "426x240" => "240p",
                    "640x360" => "360p",
                    "854x480" => "480p",
                    "1280x720" => "720p",
                    "1920x1080" => "1080p",
                    "2560x1140" => "2K",
                    "3840x2160" => "4K",
                    _ => resolution.as_str(),
                };

                if let Some(pband) = res_band.get(res_fmt) {
                    if bandwidth > pband.0 {
                        res_band.insert(res_fmt, (bandwidth, i));
                    }
                } else {
                    res_band.insert(res_fmt, (bandwidth, i));
                }

                streams.push(format!(
                    "{:#2}) {:#9} {:>6} {}/s",
                    i + 1,
                    res_fmt,
                    band_fmt.0,
                    band_fmt.1,
                ));
            } else {
                streams.push(format!(
                    "{:#2}) {:#9} {:>6} {}/s",
                    i + 1,
                    "?p",
                    band_fmt.0,
                    band_fmt.1,
                ));
            }
        }

        let uri = match self.args.quality {
            Quality::SD => quality_selector("480p", res_band, &master)?,
            Quality::HD => quality_selector("720p", res_band, &master)?,
            Quality::FHD => quality_selector("1080p", res_band, &master)?,
            Quality::UHD => quality_selector("2K", res_band, &master)?,
            Quality::UHD4K => quality_selector("4K", res_band, &master)?,
            Quality::Select => {
                let index = select("Select one variant stream:".to_string(), streams)?;
                master.variants[index].uri.clone()
            }

            Quality::Max => {
                let mut index = 0;
                let mut factor = 0;

                for (i, variant) in master.variants.iter().enumerate() {
                    if let Some(resolution) = &variant.resolution {
                        let quality = resolution
                            .split("x")
                            .map(|x| {
                                x.parse::<usize>().expect(&format!(
                                    "Couldn't parse resolution of variant playlist at index {}.",
                                    i
                                ))
                            })
                            .collect::<Vec<usize>>()
                            .iter()
                            .sum::<usize>()
                            + variant.bandwidth.parse::<usize>().context(format!(
                                "Couldn't parse bandwidth of variant playlist at index {}.",
                                i
                            ))?;

                        if quality > factor {
                            factor = quality;
                            index = i.to_owned();
                        }
                    }
                }

                master.variants[index].uri.clone()
            }
        };

        Ok(self.get_url(&uri)?)
    }

    fn parse_alternative(&mut self, master: &m3u8_rs::MasterPlaylist) -> Result<()> {
        let mut audio_stream = false;
        let mut subtitle_stream = false;

        for alternative in &master.alternatives {
            match alternative.media_type {
                m3u8_rs::AlternativeMediaType::Audio => {
                    if alternative.autoselect {
                        if let Some(uri) = &alternative.uri {
                            println!("Re-targeting to download audio stream.");

                            let args = self.args.clone();
                            let tempfile = format!(
                                "{}_audio.ts",
                                self.determine_output().trim_end_matches(".ts")
                            );
                            self.args.output = None;
                            self.args.input = self.get_url(uri).unwrap();

                            let content =
                                self.downloader.get_bytes(self.args.input.clone()).unwrap();
                            match m3u8_rs::parse_playlist_res(&content).unwrap() {
                                m3u8_rs::Playlist::MediaPlaylist(meadia) => {
                                    self.download(&meadia.segments, tempfile)?;
                                }
                                _ => (),
                            }

                            audio_stream = true;
                            self.args = args;
                        }
                    }
                }

                m3u8_rs::AlternativeMediaType::Subtitles
                | m3u8_rs::AlternativeMediaType::ClosedCaptions => {
                    if alternative.autoselect {
                        if let Some(uri) = &alternative.uri {
                            println!("Re-targeting to download subtitle stream.");

                            let args = self.args.clone();
                            let tempfile = format!(
                                "{}_subtitles.vtt",
                                self.determine_output().trim_end_matches(".ts")
                            );
                            self.args.output = Some(format!(
                                "{}_subtitles.srt",
                                self.determine_output().trim_end_matches(".ts")
                            ));
                            self.args.input = self.get_url(uri).unwrap();

                            let content =
                                self.downloader.get_bytes(self.args.input.clone()).unwrap();
                            match m3u8_rs::parse_playlist_res(&content).unwrap() {
                                m3u8_rs::Playlist::MediaPlaylist(meadia) => {
                                    self.download(&meadia.segments, tempfile)?;
                                }
                                _ => (),
                            }

                            subtitle_stream = true;
                            self.args = args;
                        }
                    }
                }

                _ => (),
            }
        }

        self.audio_stream = audio_stream;
        self.subtitle_stream = subtitle_stream;
        Ok(())
    }

    pub fn segments(&mut self) -> Result<Vec<m3u8_rs::MediaSegment>> {
        if self.is_scrapable() {
            self.scrape_website()?;
        }

        let content = if self.args.input.starts_with("http") {
            self.downloader.get_bytes(self.args.input.clone())?
        } else {
            std::fs::read_to_string(self.args.input.clone())
                .context(format!("Failed to read `{}`", self.args.input))?
                .as_bytes()
                .to_vec()
        };

        match m3u8_rs::parse_playlist_res(&content).unwrap() {
            m3u8_rs::Playlist::MasterPlaylist(master) => {
                self.args.input = self.parse_master(&master)?;
                println!("Input was switched to {}", self.args.input);

                self.parse_alternative(&master)?;

                let playlist = self.downloader.get_bytes(self.args.input.clone()).unwrap();
                match m3u8_rs::parse_playlist_res(&playlist).unwrap() {
                    m3u8_rs::Playlist::MediaPlaylist(meadia) => {
                        return Ok(meadia.segments);
                    }
                    _ => bail!("Media playlist not found."),
                }
            }
            m3u8_rs::Playlist::MediaPlaylist(meadia) => {
                return Ok(meadia.segments);
            }
        }
    }

    pub fn determine_output(&self) -> String {
        let path = if let Some(output) = self.args.input.split("/").find(|x| x.ends_with(".m3u8")) {
            crate::path::replace_ext(output.split("?").next().unwrap(), "ts")
        } else {
            "merged.ts".to_owned()
        };

        if std::path::Path::new(&path).exists() {
            let stemed_path = std::path::Path::new(&path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();

            for i in 1..100 {
                let core_file_copy = format!("{} ({}).ts", stemed_path, i);

                if !std::path::Path::new(&core_file_copy).exists() {
                    return core_file_copy;
                }
            }
        }

        path
    }

    pub fn download(
        &self,
        segments: &Vec<m3u8_rs::MediaSegment>,
        mut tempfile: String,
    ) -> Result<()> {
        if let Some(output) = &self.args.output {
            if output.ends_with(".ts") {
                tempfile = output.clone();
            }
            println!("File will be saved at {}", tempfile);
        } else {
            println!("Temporary file will be saved at {}", tempfile);
        }

        let total = segments.len();

        let pb = Arc::new(Mutex::new(kdam::tqdm!(
            total = total,
            unit = "ts".to_owned()
        )));
        pb.lock().unwrap().refresh();

        let merger = Arc::new(Mutex::new(crate::merger::BinarySequence::new(
            total,
            tempfile.clone(),
        )));

        let client = Arc::new(self.downloader.clone());
        let pool = threadpool::ThreadPool::new(self.args.threads as usize);

        for (i, segment) in segments.iter().enumerate() {
            let pb = pb.clone();
            let merger = merger.clone();
            let client = client.clone();
            let uri = self.get_url(&segment.uri)?;
            let total_retries = self.args.retry_count.clone();
            let mut retries = 0;
            let byterange = segment.byte_range.clone();
            let key = segment.key.clone();

            let key_uri = match &segment.key {
                Some(m3u8_rs::Key {
                    uri: Some(link), ..
                }) => Some(self.get_url(&link)?),
                _ => None,
            };

            pool.execute(move || {
                let mut data = loop {
                    let resp = match byterange {
                        Some(m3u8_rs::ByteRange {
                            length: start,
                            offset: Some(end),
                        }) => client.get_bytes_range(uri.clone(), start, start + end - 1),
                        _ => client.get_bytes(uri.clone()),
                    };

                    if resp.is_ok() {
                        break resp.unwrap();
                    } else {
                        if total_retries > retries {
                            pb.lock().unwrap().write(format!(
                                "{} {}",
                                "Retrying:".colorize("bold yellow"),
                                uri
                            ));
                            retries += 1;
                            continue;
                        } else {
                            pb.lock().unwrap().write(format!(
                                "{} Reached maximum number of retries for {}",
                                "Error:".colorize("bold red"),
                                uri
                            ));
                            std::process::exit(1);
                        }
                    }
                };

                if let Some(eku) = key_uri {
                    data = crate::decrypt::HlsDecrypt::from_key(
                        key.unwrap(),
                        client.get_bytes(eku).unwrap(),
                    )
                    .decrypt(&data);
                }

                let mut merger = merger.lock().unwrap();
                merger.write(i, &data).unwrap();
                merger.flush().unwrap();

                let mut pb = pb.lock().unwrap();

                pb.set_description(format!(
                    "{} / {}",
                    format_bytes(merger.stored()).2,
                    format_bytes(merger.estimate()).2
                ));

                pb.update(1);
            });
        }

        pool.join();
        eprint!("\n");
        merger.lock().unwrap().flush().unwrap();

        if merger.lock().unwrap().buffered() {
            println!("File {} downloaded successfully.", tempfile);
        } else {
            bail!("File {} is not downloaded successfully.", tempfile);
        }

        if let Some(output) = &self.args.output {
            let audio_file = format!("{}_audio.ts", tempfile.trim_end_matches(".ts"));
            let subtitle_file = format!("{}_subtitles.srt", tempfile.trim_end_matches(".ts"));
            let mut args = vec!["-i", &tempfile];

            if self.audio_stream {
                args.push("-i");
                args.push(&audio_file);
            }

            if self.subtitle_stream {
                args.push("-i");
                args.push(&subtitle_file);
            }

            if std::path::Path::new(output).exists() {
                std::fs::remove_file(output)?;
            }

            args.push("-c");
            args.push("copy");
            args.push(output);

            println!("Executing `ffmpeg {}`", args.join(" "));
            std::process::Command::new("ffmpeg")
                .args(args)
                .stderr(std::process::Stdio::null())
                .spawn()?
                .wait()?;

            if self.audio_stream {
                std::fs::remove_file(&audio_file)?;
            }

            if self.subtitle_stream {
                std::fs::remove_file(&subtitle_file)?;
            }

            std::fs::remove_file(tempfile)?;
        }
        Ok(())
    }
}
