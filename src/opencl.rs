use crate::hasty_impl;

pub fn configure_opencl(verbose: bool, ask: bool) {
    unsafe {
        crate::hasty_impl::configureOpenCL(verbose, ask);
    }
}