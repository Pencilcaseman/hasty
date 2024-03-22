#ifndef HASTY_IMPL_OPENCL_ERRORS_HPP
#define HASTY_IMPL_OPENCL_ERRORS_HPP
#ifdef HASTY_IMPL_HAS_OPENCL

#include "opencl/opencl.hpp"
#include "clblast.h"

const char *getOpenCLErrorString(int64_t error);
const char *getCLBlastErrorString(clblast::StatusCode status);

#endif // HASTY_IMPL_HAS_OPENCL
#endif // HASTY_IMPL_OPENCL_ERRORS_HPP