extern crate pkg_config;
extern crate bindgen;
extern crate regex;

use std::env;
use std::path::PathBuf;

use regex::Regex;
use bindgen::callbacks::{IntKind, ParseCallbacks};

#[derive(Debug)]
struct IntCallbacks;

impl ParseCallbacks for IntCallbacks {
    fn int_macro(&self, _name: &str, value: i64) -> Option<IntKind> {
        let ch_layout = Regex::new(r"^AV_CH").unwrap();
        let codec_cap = Regex::new(r"^AV_CODEC_CAP").unwrap();
        let codec_flag = Regex::new(r"^AV_CODEC_FLAG").unwrap();
        let error_max_size = Regex::new(r"^AV_ERROR_MAX_STRING_SIZE").unwrap();

        if value >= i64::min_value() as i64 && value <= i64::max_value() as i64 &&
           ch_layout.is_match(_name) {
            Some(IntKind::ULongLong)
        } else if value >= i32::min_value() as i64 && value <= i32::max_value() as i64 &&
                  (codec_cap.is_match(_name) || codec_flag.is_match(_name)) {
            Some(IntKind::UInt)
            //TODO: Fix usize checks
        } else if
        //value >= usize::min_value() as i64 && value <= usize::max_value() as i64 &&
        error_max_size.is_match(_name) {
            Some(IntKind::Custom {
                     name: "usize",
                     is_signed: false,
                 })
        } else if value >= i32::min_value() as i64 && value <= i32::max_value() as i64 {
            Some(IntKind::Int)
        } else {
            None
        }
    }
}

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .no_unstable_rust()
        .ctypes_prefix("libc")
        .hide_type("AV_CH_LAYOUT_NATIVE")
        .hide_type("AVRational")
        .parse_callbacks(Box::new(IntCallbacks))
        // The input header we would like to generate
        // bindings for.
        .header("src/ffmpeg/libavcodec/avcodec.h")
        .header("src/ffmpeg/libavcodec/options_table.h")

        .header("src/ffmpeg/libavformat/avformat.h")

        .header("src/ffmpeg/libavutil/rational.h")
        .header("src/ffmpeg/libavutil/avutil.h")
        .header("src/ffmpeg/libavutil/pixfmt.h")
        .header("src/ffmpeg/libavutil/time.h")

        .header("src/ffmpeg/libavfilter/buffersrc.h")
        .header("src/ffmpeg/libavfilter/avfilter.h")
        .header("src/ffmpeg/libavfilter/buffersink.h")

        .header("src/ffmpeg/libswresample/swresample.h")

        .header("src/ffmpeg/libswscale/swscale.h")

        .header("src/ffmpeg/libavdevice/avdevice.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let statik = env::var("CARGO_FEATURE_STATIC").is_ok();

    pkg_config::Config::new()
        .statik(statik)
        .probe("libavcodec")
        .unwrap()
        .include_paths;

    pkg_config::Config::new()
        .statik(statik)
        .probe("libavfilter")
        .unwrap()
        .include_paths;

    pkg_config::Config::new()
        .statik(statik)
        .probe("libavdevice")
        .unwrap()
        .include_paths;
}
