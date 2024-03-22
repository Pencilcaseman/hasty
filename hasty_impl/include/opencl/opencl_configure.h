#ifndef HASTY_IMPL_CONFIGURE_OPENCL_HPP
#define HASTY_IMPL_CONFIGURE_OPENCL_HPP
#ifdef HASTY_IMPL_HAS_OPENCL

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

void configureOpenCL(bool verbose, bool ask);

#ifdef __cplusplus
}
#endif

#endif // HASTY_IMPL_HAS_OPENCL
#endif // HASTY_IMPL_CONFIGURE_OPENCL_HPP