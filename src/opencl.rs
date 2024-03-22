use crate::hasty_impl;

/// Configure OpenCL -- Temporary function. This will be replaced soon
pub fn configure_opencl(verbose: bool, ask: bool) {
    unsafe {
        crate::hasty_impl::configureOpenCL(verbose, ask);
    }
}