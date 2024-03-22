#ifdef HASTY_IMPL_HAS_OPENCL

#include <vector>
#include <iostream>
#include <map>
#include "hasty_opencl.h"
#include "opencl/opencl.hpp"
#include "opencl/opencl_errors.hpp"
#include "opencl/opencl_global.hpp"
#include "clblast.h"

#ifdef __cplusplus
extern "C" {
#endif

struct OpenCLTestResult {
    bool pass;
    int64_t err;
    std::string errStr;
    std::string buildLog;
};

int64_t openclDeviceCompute(const cl::Device &device) {
    cl_uint computeUnits = device.getInfo<CL_DEVICE_MAX_COMPUTE_UNITS>();
    cl_uint clockFreq = device.getInfo<CL_DEVICE_MAX_CLOCK_FREQUENCY>();
    cl_ulong globalMemSize = device.getInfo<CL_DEVICE_GLOBAL_MEM_SIZE>();
    cl_device_type deviceType = device.getInfo<CL_DEVICE_TYPE>();
    std::string vendorName = device.getInfo<CL_DEVICE_VENDOR>();

    int64_t typeScore = (deviceType == CL_DEVICE_TYPE_GPU) ? 1000000 : 0;
    int64_t cudaScore = (vendorName.find("NVIDIA") != std::string::npos) ? 1000000 : 0;
    int64_t memScore = globalMemSize / (1024 * 1024);

    return static_cast<int64_t>(computeUnits * clockFreq) + typeScore + cudaScore + memScore;
}

OpenCLTestResult testOpenCLDevice(const cl::Device &device) {
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
            return {false,
                    err,
                    getOpenCLErrorString(err),
                    program.getBuildInfo<CL_PROGRAM_BUILD_LOG>(device)};
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

        bool pass = dst == std::vector<float>({6, 6, 6, 6, 6});
        return {pass, 0, "UNKNOWN_ERROR", ""};
    } catch (const std::exception &e) {
        return {
                false,
                -1,
                e.what(),
                "UNKNOWN_ERROR",
        };
    }
}

void updateOpenCLDevices(bool verbose) {
    std::vector<cl::Platform> platforms;
    cl::Platform::get(&platforms);

    for (const auto &platform: platforms) {
        std::vector<cl::Device> devices;
        platform.getDevices(CL_DEVICE_TYPE_ALL, &devices);
        if (!devices.empty()) {
            if (verbose) {
                std::cout << "Platform: " << platform.getInfo<CL_PLATFORM_NAME>() << "\n";
                std::cout << "  Vendor: " << platform.getInfo<CL_PLATFORM_VENDOR>() << "\n";
                std::cout << "  Version: " << platform.getInfo<CL_PLATFORM_VERSION>() << "\n";
            }

            for (auto &device: devices) {
                // Test the device to check it works
                auto [pass, err, errStr, buildLog] = testOpenCLDevice(device);
                if (verbose) {
                    std::cout << "  Device [id=" << openclDevices.size() << "]: "
                              << device.getInfo<CL_DEVICE_NAME>()
                              << (pass ? "" : " [ FAILED ]") << "\n";

                    auto computeUnits = device.getInfo<CL_DEVICE_MAX_COMPUTE_UNITS>();
                    auto clocFreq = device.getInfo<CL_DEVICE_MAX_CLOCK_FREQUENCY>();
                    auto memory =
                            (device.getInfo<CL_DEVICE_GLOBAL_MEM_SIZE>() + (1 << 30)) / (1 << 30);
                    auto version = device.getInfo<CL_DEVICE_VERSION>();
                    auto profile = device.getInfo<CL_DEVICE_PROFILE>();

                    std::cout << "    Compute Units: " << computeUnits << "\n";
                    std::cout << "    Clock:         " << clocFreq << "MHz\n";
                    std::cout << "    Memory:        " << memory << "GB\n";
                    std::cout << "    Version:       " << version << "\n";
                    std::cout << "    Profile:       " << profile << "\n";
                    std::cout << "    Compute Score: " << openclDeviceCompute(device) << "\n";
                }

                if (!pass) continue;

                openclDevices.push_back(device);
            }
        }
    }
}

cl::Device findFastestDevice(const std::vector<cl::Device> &devices) {
    cl::Device fastest;
    int64_t fastestCompute = 0;
    for (auto &device: devices) {
        int64_t compute = openclDeviceCompute(device);
        if (compute > fastestCompute) {
            fastestCompute = compute;
            fastest = device;
        }
    }
    return fastest;
}

void configureOpenCL(bool verbose, bool ask) {
    if (verbose) {
        std::cout << "============== OpenCL Configuration ==============\n";
    }

    updateOpenCLDevices(verbose);

    if (!ask) {
        // Select the fastest device by default
        openCLDevice = findFastestDevice(openclDevices);
    } else {
        // Otherwise, prompt the user to select a device
        int64_t deviceIndex = -1;
        while (deviceIndex < 0 || deviceIndex >= int64_t(openclDevices.size())) {
            std::cout << "Select OpenCL device [0-" << openclDevices.size() - 1 << "]: ";
            std::cout << std::flush;
            std::cin >> deviceIndex;
        }

        openCLDevice = openclDevices[deviceIndex];
    }

    if (verbose) {
        std::string deviceDetails = "Selected Device: {}" + openCLDevice.getInfo<CL_DEVICE_NAME>();
        std::cout << deviceDetails << "\n";
    }

    openCLContext = cl::Context(openCLDevice);
    openCLQueue = cl::CommandQueue(openCLContext, openCLDevice);
}

#ifdef __cplusplus
}
#endif

#else

void opencl_configure_placeholder() {}

#endif // HASTY_IMPL_HAS_OPENCL