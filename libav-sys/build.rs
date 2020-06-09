use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn fmt_write(builder: bindgen::Builder) -> String {
    builder
        .generate()
        .unwrap()
        .to_string()
        .replace("pub const FP_NAN: _bindgen_ty_1 = 0;", "")
        .replace("pub const FP_INFINITE: _bindgen_ty_1 = 1;", "")
        .replace("pub const FP_ZERO: _bindgen_ty_1 = 2;", "")
        .replace("pub const FP_SUBNORMAL: _bindgen_ty_1 = 3;", "")
        .replace("pub const FP_NORMAL: _bindgen_ty_1 = 4;", "")
        .replace("/**", "/*")
        .replace("/*!", "/*")
}

fn main() {
    let libs = metadeps::probe().unwrap();
    let openh264 = libs.get("libavcodec").unwrap();
    let headers = openh264.include_paths.clone();

    let mut builder = bindgen::builder()
        .header("data/libavcodec.h")
        .derive_default(true)
        .derive_debug(true);

    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    let s = fmt_write(builder);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = File::create(out_path.join("libavcodec.rs")).unwrap();
    let _ = file.write(s.as_bytes());
}
