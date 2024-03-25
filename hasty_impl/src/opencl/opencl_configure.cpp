#ifdef HASTY_IMPL_HAS_OPENCL

#include <vector>
#include <iostream>
#include "hasty_opencl.h"
#include "opencl/opencl.hpp"
#include "opencl/opencl_errors.hpp"
#include "opencl/opencl_global.hpp"
#include "clblast.h"

int64_t openclDeviceCompute(const cl::Device &device) {
    cl_uint computeUnits = device.getInfo<CL_DEVICE_MAX_COMPUTE_UNITS>();
    cl_uint clockFreq = device.getInfo<CL_DEVICE_MAX_CLOCK_FREQUENCY>();
    cl_ulong globalMemSize = device.getInfo<CL_DEVICE_GLOBAL_MEM_SIZE>();
    cl_device_type deviceType = device.getInfo<CL_DEVICE_TYPE>();
    std::string vendorName = device.getInfo<CL_DEVICE_VENDOR>();

    int64_t typeScore = (deviceType == CL_DEVICE_TYPE_GPU) ? 1000000 : 0;
    int64_t cudaScore = (vendorName.find("NVIDIA") != std::string::npos) ? 1000000 : 0;
    int64_t memScore = static_cast<int64_t>(globalMemSize) / (1024 * 1024);

    return static_cast<int64_t>(computeUnits * clockFreq) + typeScore + cudaScore + memScore;
}

bool testOpenCLDevice(const cl::Device &device) {
    try {
        cl::Context context(device);
        cl::CommandQueue queue(context, device);

        std::string source = R"V0G0N(
__kernel void testAddition(__global const float *a, __global const float *b, __global float *c) {
const int i = get_global_id(0);
c[i] = a[i] + b[i];
}
)V0G0N";
        cl::Program::Sources sources;
        sources.emplace_back(source.c_str(), source.length() + 1);

        cl_int err;
        cl::Program program(context, sources);
        err = program.build();

        // Check the build status
        cl_build_status buildStatus = program.getBuildInfo<CL_PROGRAM_BUILD_STATUS>(device);

        if (buildStatus != CL_BUILD_SUCCESS) {
            return false;
        }

        std::vector<float> srcA = {1, 2, 3, 4, 5};
        std::vector<float> srcB = {5, 4, 3, 2, 1};
        std::vector<float> dst(5);
        size_t numElements = srcA.size();
        cl::Buffer bufA(context, CL_MEM_READ_ONLY, numElements * sizeof(float));
        cl::Buffer bufB(context, CL_MEM_READ_ONLY, numElements * sizeof(float));
        cl::Buffer bufC(context, CL_MEM_WRITE_ONLY, numElements * sizeof(float));

        queue.enqueueWriteBuffer(bufA, CL_TRUE, 0, numElements * sizeof(float), srcA.data());
        queue.enqueueWriteBuffer(bufB, CL_TRUE, 0, numElements * sizeof(float), srcB.data());

        cl::Kernel kernel(program, "testAddition");
        kernel.setArg(0, bufA);
        kernel.setArg(1, bufB);
        kernel.setArg(2, bufC);

        cl::NDRange global_size(numElements);
        queue.enqueueNDRangeKernel(kernel, cl::NullRange, global_size, cl::NullRange);
        queue.enqueueReadBuffer(bufC, CL_TRUE, 0, numElements * sizeof(float), dst.data());

        return dst == std::vector<float>({6, 6, 6, 6, 6});
    } catch (const std::exception &e) {
        return false;
    }
}

void updateOpenCLDevices() {
    std::vector<cl::Platform> platforms;
    cl::Platform::get(&platforms);
    global::openclDevices.clear();

    for (const auto &platform: platforms) {
        std::vector<cl::Device> devices;
        platform.getDevices(CL_DEVICE_TYPE_ALL, &devices);
        if (!devices.empty()) {
            for (auto &device: devices) {
                // Test the device to check it works
                if (!testOpenCLDevice(device)) continue;
                global::openclDevices.push_back(device);
            }
        }
    }
}

cl::Device findFastestDevice(const std::vector<cl::Device> &devices) {
    cl::Device fastest;
    int64_t fastestCompute = 0;

    for (const auto &device: devices) {
        int64_t compute = openclDeviceCompute(device);
        if (compute > fastestCompute) {
            fastestCompute = compute;
            fastest = device;
        }
    }
    return fastest;
}

#ifdef __cplusplus
extern "C" {
#endif
void configureOpenCL() {
    updateOpenCLDevices();

    if (global::openclDevices.empty()) {
        std::cerr << "Failed to find an OpenCL-compatible device";
        exit(1);
    }

    global::openCLDevice = findFastestDevice(global::openclDevices);
    global::openCLContext = cl::Context(global::openCLDevice);
    global::openCLQueue = cl::CommandQueue(global::openCLContext, global::openCLDevice);
}

#ifdef __cplusplus
}
#endif

#else

void opencl_configure_placeholder() {}

#endif // HASTY_IMPL_HAS_OPENCL