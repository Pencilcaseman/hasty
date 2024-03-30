#ifdef HASTY_IMPL_HAS_OPENCL

#include <vector>
#include <iostream>
#include <hasty_opencl.h>
#include <opencl/opencl.hpp>
#include <opencl/opencl_errors.hpp>
#include <opencl/opencl_global.hpp>
#include <opencl/opencl_kernel.hpp>
#include <clblast.h>

OpenCLErrorCode opencl_add_kernel(const std::string &kernelSource) {
    global::openCLSources.push_back({kernelSource.c_str(), kernelSource.length()});
    global::openCLProgram = cl::Program(global::openCLContext, global::openCLSources);
    return get_opencl_error_code(global::openCLProgram.build());
}

#ifdef __cplusplus
extern "C" {
#endif

enum OpenCLErrorCode opencl_add_kernel_ffi(const char *kernelSource) {
    return opencl_add_kernel(std::string(kernelSource));
}

enum OpenCLErrorCode
opencl_run_contiguous_linear_kernel_3_ffi(const char *kernelName, uint64_t numElements, void *buf0, void *buf1,
                                          void *buf2) {
    return opencl_run_contiguous_linear_kernel(std::string(kernelName), numElements, *(cl::Buffer *) buf0,
                                               *(cl::Buffer *) buf1, *(cl::Buffer *) buf2);
}

#ifdef __cplusplus
}
#endif

#else

void opencl_kernel_placeholder() {}

#endif // HASTY_IMPL_HAS_OPENCL