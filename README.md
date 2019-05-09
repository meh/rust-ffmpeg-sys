# FFmpeg binding for Rust

## Quick Start

If setting environment variable `FFMPEG_VERSION` to X.X.X will fetch FFmpeg source from Github and build from it.
If setting environment variable `FFMPEG_DIR` to path that FFmpeg installed will skip build and link to it directly.

Note that version must be full version included patch number. If none of above is set it will fetch FFmpeg which version matched ffmpeg-sys.
 