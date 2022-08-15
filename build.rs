use cmake;
use std::{env, fs, path};

fn main() {
    let mut klusolvex_config = cmake::Config::new("dss_capi/klusolvex");
    klusolvex_config.define("USE_SYSTEM_SUITESPARSE", "OFF"); // FIXME: should be "ON", but causes failure..
    klusolvex_config.define("USE_SYSTEM_EIGEN3", "ON");

    let klusolve_lib_type = "SHARED";
    klusolvex_config.define("KLUSOLVE_LIB_TYPE", klusolve_lib_type);
    klusolvex_config.define("CMAKE_BUILD_TYPE", "Release");
    klusolvex_config.build();

    let mut klusolvex_config = cmake::Config::new("dss_capi/klusolvex");
    klusolvex_config.configure_arg("--build");
    klusolvex_config.configure_arg(".");
    klusolvex_config.build();

    let klusolvex_lib_src = "dss_capi/klusolvex/lib/linux_x64/libklusolvexd.so";
    let klusolvex_lib_dst = "dss_capi/lib/linux_x64/libklusolvexd.so";
    fs::copy(klusolvex_lib_src, klusolvex_lib_dst).expect("Failed to copy klusolvex");

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

    println!("cargo:rustc-link-lib=dss_capi");
    println!("cargo:rustc-link-search=dss_capi/lib/linux_x64");
}
