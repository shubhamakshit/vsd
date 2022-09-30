<h1 align="center">vsd</h1>

<p align="center">
  <a href="https://github.com/clitic/vsd">
    <img src="https://img.shields.io/github/downloads/clitic/vsd/total?logo=github&style=flat-square">
  </a>
  <a href="https://crates.io/crates/vsd">
    <img src="https://img.shields.io/crates/d/vsd?logo=rust&style=flat-square">
  </a>
  <a href="https://crates.io/crates/vsd">
    <img src="https://img.shields.io/crates/v/vsd?style=flat-square">
  </a>
  <a href="https://docs.rs/vsd/latest/vsd">
    <img src="https://img.shields.io/docsrs/vsd?logo=docsdotrs&style=flat-square">
  </a>
  <a href="https://github.com/clitic/vsd">
    <img src="https://img.shields.io/github/license/clitic/vsd?style=flat-square">
  </a>
  <a href="https://github.com/clitic/vsd">
    <img src="https://img.shields.io/github/repo-size/clitic/vsd?logo=github&style=flat-square">
  </a>
  <a href="https://github.com/clitic/vsd">
    <img src="https://img.shields.io/tokei/lines/github/clitic/vsd?style=flat-square">
  </a>
  <a href="https://colab.research.google.com/github/clitic/vsd/blob/main/vsd-on-colab.ipynb">
    <img src="https://img.shields.io/badge/Open%20In%20Colab-F9AB00?logo=googlecolab&color=525252&style=flat-square">
  </a>
</p>

<p align="center">
  <a href="#Installations">Installations</a>
  &nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#Usage">Usage</a>
  &nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="https://colab.research.google.com/github/clitic/vsd/blob/main/vsd-on-colab.ipynb">Try Without Install</a>
</p>

Command line program to download HLS video from websites and m3u8 links.

Know more about HLS from [howvideo.works](https://howvideo.works) and 
[wikipedia](https://en.wikipedia.org/wiki/M3U).

There are some alternatives to vsd but they lack in some features like [N_m3u8DL-CLI](https://github.com/nilaoda/N_m3u8DL-CLI) is not cross platform and [m3u8-downloader](https://github.com/llychao/m3u8-downloader) has very few customizable options. There are also options like [webvideo-downloader](https://github.com/jaysonlong/webvideo-downloader) which open websites using chrome and captures the m3u8 links and then download it. A similar functionality can achieved with vsd too by using *capture* and *collect* features. 

<p align="center">
  <img src="https://github.com/clitic/vsd/blob/main/images/showcase.png">
</p>

## Features

- [x] Captures m3u8 network requests from websites.
- [x] Collects .m3u8, .mpd and subtitles from websites and save them locally.
- [x] Custom headers, proxies and cookies.
- [x] Inbuilt web scrapper for querying HLS and DASH links.
- [x] Human friendly resolution and bandwidth based master playlist variants parsing.
- [x] Multiple output formats which are supported by ffmpeg.
- [x] Mux seperate video, audio and subtitle (webvtt) stream to a single file.
- [x] Progressive binary merging of segments.
- [x] Realtime file size estimation.
- [x] Select standard resolution playlists like `HD`, `FHD` etc.
- [x] Supports `AES-128` playlist decryption.
- [x] Supports downloading in multiple threads.
- [x] Supports resume and retries.
- [ ] GUI
- [ ] Supports Dash (soon)
- [ ] Supports [SAMPLE-AES](https://developer.apple.com/library/archive/documentation/AudioVideo/Conceptual/HLS_Sample_Encryption/Encryption/Encryption.html) playlist decryption.
- [ ] Supports live stream download. (soon)

<a href="#Help">See More</a>

## Installations
  
Dependencies

- [ffmpeg](https://www.ffmpeg.org/download.html) (optional, *recommended*) only required for transmuxing and transcoding streams.
- [chrome](https://www.google.com/chrome) / [chromium](https://www.chromium.org/getting-involved/download-chromium/) (optional) only required for `capture` and `collect` subcommands. 

Visit [releases](https://github.com/clitic/vsd/releases) for prebuilt binaries. You just need to copy that binary to any path specified in your `PATH` environment variable.

### Through Cargo

```bash
cargo install vsd
```

### On x86_64 Linux

```bash
curl -L https://github.com/clitic/vsd/releases/download/v0.1.2/vsd-v0.1.2-x86_64-unknown-linux-musl.tar.gz | tar xz
```

### On Termux (Android 11+)

```bash
curl -L https://github.com/clitic/vsd/releases/download/v0.1.2/vsd-v0.1.2-aarch64-linux-android.tar.gz | tar xz -C $PREFIX/bin
```

```bash
# optional
pkg install ffmpeg
```

Also, see [running on android](https://github.com/clitic/vsd/blob/main/docs/running-on-android.md)

## Usage

For quick testing purposes you may use [https://test-streams.mux.dev](https://test-streams.mux.dev) as direct input. These streams are used by [hls.js](https://github.com/video-dev/hls.js) for testing purposes.

- Downloading HLS video from a website, m3u8 url or from a local m3u8 file.

```bash
$ vsd <url | .m3u8> -o video.mp4
```

- Collecting .m3u8 (HLS), .mpd (Dash) and subtitles from a website and saving them locally.

```bash
$ vsd <url> --collect
```

## Help

```bash
$ vsd --help
```

```
vsd 0.1.2
clitic <clitic21@gmail.com>
Command line program to download HLS video from websites and m3u8 links.

USAGE:
    vsd.exe [OPTIONS] <INPUT>

ARGS:
    <INPUT>    url | .m3u8 | .m3u

OPTIONS:
    -a, --alternative                  Download alternative streams such as audio and subtitles
                                       streams from master playlist instead of variant video streams
    -b, --baseurl <BASEURL>            Base url for all segments. Usually needed for local m3u8 file
    -h, --help                         Print help information
    -o, --output <OUTPUT>              Path of final downloaded video stream. For file extension any
                                       ffmpeg supported format could be provided. If playlist
                                       contains alternative streams vsd will try to transmux and
                                       trancode into single file using ffmpeg
    -q, --quality <QUALITY>            Automatic selection of some standard resolution streams with
                                       highest bandwidth stream variant from master playlist
                                       [default: select] [possible values: select, sd, hd, fhd, uhd,
                                       uhd4k, max]
    -r, --resume                       Resume a download session. Download session can only be
                                       resumed if download session json file is present
        --raw-prompts                  Raw style input prompts for old and unsupported terminals
        --retry-count <RETRY_COUNT>    Maximum number of retries to download an individual segment
                                       [default: 15]
    -s, --skip                         Skip downloading and muxing alternative streams
    -t, --threads <THREADS>            Maximum number of threads for parllel downloading of
                                       segments. Number of threads should be in range 1-16
                                       (inclusive) [default: 5]
    -V, --version                      Print version information

CHROME OPTIONS:
        --build       Build http links for all uri present in .m3u8 file while collecting it.
                      Resultant .m3u8 file can be played and downloaded directly without the need of
                      `--baseurl` flag. This option should must be used with `--collect` flag only
        --capture     Launch Google Chrome to capture requests made to fetch .m3u8 (HLS) and .mpd
                      (Dash) files
        --collect     Launch Google Chrome and collect .m3u8 (HLS), .mpd (Dash) and subtitles from a
                      website and save them locally
        --headless    Launch Google Chrome without a window for interaction. This option should must
                      be used with `--capture` or `--collect` flag only

CLIENT OPTIONS:
        --cookies <cookies> <url>
            Enable cookie store and fill it with some existing cookies. Example `--cookies "foo=bar;
            Domain=yolo.local" https://yolo.local`. This option can be used multiple times

        --enable-cookies
            Enable cookie store which allows cookies to be stored

        --header <key> <value>
            Custom headers for requests. This option can be used multiple times

        --proxy-address <PROXY_ADDRESS>
            Set http or https proxy address for requests

        --user-agent <USER_AGENT>
            Update and set custom user agent for requests [default: "Mozilla/5.0 (Windows NT 10.0;
            Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36"]
```

## Building From Source

1. Install [Rust](https://www.rust-lang.org)

2. Download or clone Repository.

```bash
git clone https://github.com/clitic/vsd.git
```

3. Build Release (inside vsd directory)

### Linux

First install [openssl](https://docs.rs/openssl/latest/openssl/#automatic) library then run.

```bash
OPENSSL_STATIC=true cargo build --release
```

### Windows

Build [openssl](https://github.com/openssl/openssl) library for windows. vsd builds use openssl static build i.e. [openssl-v3.0.5-static-x86_64-windows-msvc.7z](https://drive.google.com/file/d/1LhVu97TiV4HSzxUH-rjiGXXZBs27iDbs/view?usp=sharing).

```powershell
$env:x86_64_PC_WINDOWS_MSVC_OPENSSL_DIR="C:\openssl-3.0.5-VC-WIN64A-static"
$env:x86_64_PC_WINDOWS_MSVC_OPENSSL_STATIC=$true
$env:x86_64_PC_WINDOWS_MSVC_NO_VENDOR=$true
cargo build --release
```

<!-- 
### x86_64-unknown-linux-musl (On Linux 64-bit)


```
# MUSL

# !apt install musl musl-dev musl-tools
!wget https://github.com/richfelker/musl-cross-make/archive/refs/tags/v0.9.9.tar.gz
!tar -xzf v0.9.9.tar.gz -C .
!rm v0.9.9.tar.gz

!cd musl-cross-make-0.9.9 && TARGET=x86_64-linux-musl make install
!cd musl-cross-make-0.9.9/output && tar -czf /content/musl-cross-make-v0.9.9-linux-64bit.tar.gz *
!rm -rf musl-cross-make-0.9.9
```

```
# openssl (MUSL)

# !apt install musl musl-dev musl-tools
!wget https://github.com/openssl/openssl/archive/refs/tags/openssl-3.0.5.tar.gz
!tar -xzf openssl-3.0.5.tar.gz -C .
!rm openssl-3.0.5.tar.gz

!cd openssl-openssl-3.0.5 && \
	CC=/content/musl-cross-make-v0.9.9/bin/x86_64-linux-musl-gcc \
	perl Configure linux-x86_64 no-shared --prefix=/content/openssl-build && \
  make && make install_sw

!cd openssl-build && tar -czf /content/openssl-v3.0.5-x86_64-linux-musl-static.tar.gz *
!rm -rf openssl-openssl-3.0.5 openssl-build
```

```
# openssl (Android 11+)

!wget https://github.com/openssl/openssl/archive/refs/tags/openssl-3.0.5.tar.gz
!tar -xzf openssl-3.0.5.tar.gz -C .
!rm openssl-3.0.5.tar.gz

cd openssl-openssl-3.0.5 && \
	ANDROID_NDK_ROOT=/content/android-ndk-r25 && \
	PATH=$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin:$ANDROID_NDK_ROOT/toolchains/arm-linux-androideabi-4.9/prebuilt/linux-x86_64/bin:$PATH && \
	perl Configure android-arm64 no-shared --prefix=/content/openssl-build --openssldir=/content/openssl-build -D__ANDROID_API__=30 && \
	make && make install_sw

!cd openssl-build && tar -czf /content/openssl-v3.0.5-android-arm64-android30-static.tar.gz *
!rm -rf openssl-openssl-3.0.5 openssl-build
```

# MUSL (Prebuilt)
!mkdir musl-cross-make-v0.9.9
!tar -xzf /content/drive/MyDrive/musl-cross-make-v0.9.9-linux-64bit.tar.gz -C musl-cross-make-v0.9.9

# openssl (Prebuilt)
!mkdir openssl-v3.0.5
!tar -xzf /content/drive/MyDrive/openssl-v3.0.5-x86_64-linux-musl-static.tar.gz -C openssl-v3.0.5

3. Add build target x86_64-unknown-linux-musl.

```bash
$ rustup target add x86_64-unknown-linux-musl
$ printf '\n[target.x86_64-unknown-linux-musl]\nlinker = "x86_64-linux-musl-gcc"\n' >> ~/.cargo/config.toml
```

```bash
$ PATH=musl-cross-make-v0.9.9/bin:$PATH \
    CC=x86_64-linux-musl-gcc \
    CXX=x86_64-linux-musl-g++ \
    PKG_CONFIG_ALLOW_CROSS=1 \
    OPENSSL_DIR=openssl-v3.0.5 \
    OPENSSL_STATIC=true \
    OPENSSL_NO_VENDOR=true \
    cargo build --release --target x86_64-unknown-linux-musl
```

!cd ./vsd/target/x86_64-unknown-linux-musl/release && tar -czf /content/vsd-v{version}-x86_64-unknown-linux-musl.tar.gz ./vsd -->

<!-- [openssl-v3.0.5-static-x86_64-linux-gnu.tar.gz](https://drive.google.com/file/d/1u7I6hNJ3P7Z6mzIQEY3VxiClJ99JbDm5/view?usp=sharing)
[openssl-v3.0.5-static-x86_64-linux-musl.tar.gz](https://drive.google.com/file/d/1V8qqgOl1fHgd2KLNplxsHgvwyvu67ITx/view?usp=sharing) -->

### Android (On Linux 64-bit)

1. Install [NDK](https://developer.android.com/ndk/downloads).

```bash
$ wget https://dl.google.com/android/repository/android-ndk-r22b-linux-x86_64.zip
$ unzip android-ndk-r22b-linux-x86_64.zip
$ rm android-ndk-r22b-linux-x86_64.zip
```

2. Build [openssl](https://github.com/openssl/openssl) library for android. vsd builds use openssl static build i.e. [openssl-v3.0.5-static-aarch64-linux-android30.tar.gz](https://drive.google.com/file/d/1Fwst1R-in2-2jGieCapeUXfLgqR2urVA/view?usp=sharing).

3. Add android target aarch64-linux-android.

```bash
$ rustup target add aarch64-linux-android
$ printf '\n[target.aarch64-linux-android]\nlinker = "aarch64-linux-android30-clang"\n' >> ~/.cargo/config.toml
```

4. Now compile with target aarch64-linux-android.

```bash
$ PATH=android-ndk-r22b/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH \
    AARCH64_LINUX_ANDROID_OPENSSL_DIR=openssl-v3.0.5-static-aarch64-linux-android30 \
    AARCH64_LINUX_ANDROID_OPENSSL_STATIC=true \
    AARCH64_LINUX_ANDROID_OPENSSL_NO_VENDOR=true \
    cargo build --release --target aarch64-linux-android
```

## License

Dual Licensed

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))
