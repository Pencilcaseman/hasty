#ifndef HASTY_IMPL_OPENCL_ERRORS_HPP
#define HASTY_IMPL_OPENCL_ERRORS_HPP
#ifdef HASTY_IMPL_HAS_OPENCL

#include <ocl/opencl.hpp>
#include <clblast.h>
#include <ocl/opencl_error_types.h>

OpenCLErrorCode get_opencl_error_code(int64_t error);
OpenCLErrorCode getCLBlastErrorCode(clblast::StatusCode status);

#endif // HASTY_IMPL_HAS_OPENCL
#endif // HASTY_IMPL_OPENCL_ERRORS_HPP