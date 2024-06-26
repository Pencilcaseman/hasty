#ifdef HASTY_IMPL_HAS_OPENCL

#include <opencl/opencl_global.hpp>

namespace global {
    std::vector<cl::Device> openclDevices;
    cl::Device openCLDevice;
    cl::Context openCLContext;
    cl::CommandQueue openCLQueue;
    cl::Program::Sources openCLSources;
    cl::Program openCLProgram;
}

#else

void opencl_global_placeholder() {}

#endif // HASTY_IMPL_HAS_OPENCL