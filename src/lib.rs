// OpenDSS `linux_x64` target.
//mod linux_x64;
//pub use crate::linux_x64::bindings::*;
#![crate_type = "lib"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
