#ifndef HASTY_IMPL_OPENCL_KERNEL_HPP
#define HASTY_IMPL_OPENCL_KERNEL_HPP
#ifdef HASTY_IMPL_HAS_OPENCL

#include <iostream>

#include "./opencl.hpp"
#include "./opencl_error_types.h"
#include "./opencl_errors.hpp"
#include "./opencl_global.hpp"
#include "./opencl_memory_impl.h"

OpenCLErrorCode opencl_add_kernel(const std::string &kernelSource);

template<size_t... I, typename... Args>
void setKernelArgs(cl::Kernel &kernel, const std::tuple<Args...> &args,
                   std::index_sequence<I...>, size_t offset = 0) {
    ((kernel.setArg(I + offset, std::get<I>(args))), ...);
}

template<typename... Args>
OpenCLErrorCode
opencl_run_contiguous_linear_kernel(const std::string &kernelName, size_t numElements,
                                    Args... args) {
//    std::cout << "Running kernel " << kernelName << " with " << numElements << " elements\n";
    cl::Kernel kernel(global::openCLProgram, kernelName.c_str());
    setKernelArgs(kernel, std::make_tuple(args...), std::make_index_sequence<sizeof...(Args)>());
    cl::NDRange range(numElements);
    return get_opencl_error_code(global::openCLQueue.enqueueNDRangeKernel(kernel, cl::NullRange, range, cl::NullRange));
}

#endif // HASTY_IMPL_HAS_OPENCL
#endif //HASTY_IMPL_OPENCL_KERNEL_HPP
