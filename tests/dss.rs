extern crate dss_rs_sys;

// Verify C API bindings can be called.
#[test]
fn unsafe_dss_start() {
    unsafe {
        let ret = dss_rs_sys::DSS_Start(0);
        assert!(ret != 0);
    }
}
