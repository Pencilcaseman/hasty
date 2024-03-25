#ifndef HASTY_IMPL_OPENCL_MEMORY_HPP
#define HASTY_IMPL_OPENCL_MEMORY_HPP
#ifdef HASTY_IMPL_HAS_OPENCL

#include "./opencl.hpp"
#include "./opencl_error_types.h"
#include "./opencl_errors.hpp"
#include "./opencl_global.hpp"
#include "./opencl_memory_impl.h"

OpenCLErrorCode opencl_allocate(size_t bytes, OpenCLMemoryType mem_type, cl::Buffer **result);

void opencl_free(cl::Buffer *buf);

OpenCLErrorCode opencl_write(const cl::Buffer &dst, const void *src, size_t bytes, bool blocking);

OpenCLErrorCode opencl_read(void *dst, const cl::Buffer &src, size_t bytes, bool blocking);

#endif // HASTY_IMPL_HAS_OPENCL
#endif //HASTY_IMPL_OPENCL_MEMORY_HPP
