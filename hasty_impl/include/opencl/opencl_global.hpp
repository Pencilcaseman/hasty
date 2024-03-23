#ifndef HASTY_IMPL_OPENCL_GLOBAL
#define HASTY_IMPL_OPENCL_GLOBAL
#ifdef HASTY_IMPL_HAS_OPENCL

#include "opencl/opencl.hpp"

namespace global {
    extern std::vector<cl::Device> openclDevices;
    extern cl::Device openCLDevice;
    extern cl::Context openCLContext;
    extern cl::CommandQueue openCLQueue;
}

#endif // HASTY_IMPL_HAS_OPENCL
#endif // HASTY_IMPL_OPENCL_GLOBAL