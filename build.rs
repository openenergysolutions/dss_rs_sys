use cmake;
use std::{env, fs, path};

fn main() {
    let mut klusolvex_config = cmake::Config::new("klusolve");
    klusolvex_config.define("USE_SYSTEM_SUITESPARSE", "OFF"); // FIXME: should be "ON", but causes failure..
    klusolvex_config.define("USE_SYSTEM_EIGEN3", "ON");

    let klusolve_lib_type = "SHARED";
    klusolvex_config.define("KLUSOLVE_LIB_TYPE", klusolve_lib_type);
    klusolvex_config.define("CMAKE_BUILD_TYPE", "Release");
    klusolvex_config.build();

    let mut klusolvex_config = cmake::Config::new("klusolve");
    klusolvex_config.configure_arg(".");
    klusolvex_config.build();

    let klusolve_lib_src = "klusolve/lib/linux_x64/libklusolvexd.so";
    let klusolve_lib_dst = "dss_capi/lib/linux_x64/libklusolvex.so";
    fs::copy(klusolve_lib_src, klusolve_lib_dst).expect("Failed to write to dss_capi/lib/linux_x64");

    let bindings = bindgen::Builder::default()
        .header("dss_capi/include/dss_capi.h")
        .header("dss_capi/include/dss_capi_ctx.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let pwd = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let mut lib = pwd;
    lib.push_str("/dss_capi/lib/linux_x64");

    println!("cargo:rustc-link-arg=-Wl,-rpath={}", lib);
    println!("cargo:rustc-link-lib=dss_capi");
}
