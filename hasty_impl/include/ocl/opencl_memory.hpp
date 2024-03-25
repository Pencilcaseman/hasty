#ifndef HASTY_IMPL_OPENCL_MEMORY_HPP
#define HASTY_IMPL_OPENCL_MEMORY_HPP
#ifdef HASTY_IMPL_HAS_OPENCL

#include <ocl/opencl.hpp>
#include <ocl/opencl_error_types.h>
#include <ocl/opencl_errors.hpp>
#include <ocl/opencl_global.hpp>
#include <ocl/opencl_memory_impl.h>

OpenCLErrorCode opencl_allocate(size_t bytes, OpenCLMemoryType mem_type, cl::Buffer **result);

void opencl_free(cl::Buffer *buf);

OpenCLErrorCode opencl_write(const cl::Buffer &dst, const void *src, size_t bytes, bool blocking);

OpenCLErrorCode opencl_read(void *dst, const cl::Buffer &src, size_t bytes, bool blocking);

#endif // HASTY_IMPL_HAS_OPENCL
#endif //HASTY_IMPL_OPENCL_MEMORY_HPP