# dss_rs_sys

Rudimentary crate providing unsafe bindings to DSS Extension's DSS C API.

This project is currently only available for 64-bit linux systems.


## Dependencies

External dependencies required to use/build by the DSS C API.

- SuiteSparse
```
sudo apt-get install -y libsuitesparse-dev
```
<sub>**_Note:_** The locally installed version of this dependency is not actually used; there is a bug that occurs during builds unless `USE_SYSTEM_SUITESPARSE=OFF`. This means that SuiteSparse is downloaded and built each time this crate is built.</sub>

- Eigen3
```
sudo apt-get install -y libeigen3-dev
```

- [Free Pascal compiler (v3.2.2)](https://www.freepascal.org/)


## Building   

`cargo build` will perform two things:

- Build `klusolvex` (an external dependency of `dss_capi`) for a `linux_x64` target.
- Produce bindings for the `dss_capi`.

After running `cargo build`, **_you must build `dss_capi` yourself_** in order to actually use the `dss_capi` library.
```
bash build/build_linux_x64.sh
```

All object files will be in `dss_capi/lib/linux_x64`. Using them requires setting `LD_LIBRARY_PATH` to include their path.