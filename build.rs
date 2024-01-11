// Copyright 2024 Open Energy Solutions Inc.
// 
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

    let klusolve_lib_src = "klusolve/lib/linux_x64/libklusolvex.so";
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
    println!("cargo:rustc-link-arg=-Wl,-rpath=/usr/local/lib");
    println!("cargo:rustc-link-lib=dss_capi");
}
