#ifndef HASTY_IMPL_OPENCL_KERNEL_IMPL_H
#define HASTY_IMPL_OPENCL_KERNEL_IMPL_H
#ifdef HASTY_IMPL_HAS_OPENCL

#include "./opencl_error_types.h"
#include "./opencl_memory_impl.h"

enum OpenCLErrorCode opencl_add_kernel_ffi(const char *kernelSource);

enum OpenCLErrorCode
opencl_run_contiguous_linear_kernel_3_ffi(const char *kernelName, uint64_t numElements, void *buf0, void *buf1,
                                          void *buf2);

#endif // HASTY_IMPL_HAS_OPENCL
#endif //HASTY_IMPL_OPENCL_KERNEL_IMPL_H
