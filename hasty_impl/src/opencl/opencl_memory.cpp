#ifdef HASTY_IMPL_HAS_OPENCL

#include <opencl/opencl_memory.hpp>

OpenCLErrorCode opencl_allocate(uint64_t bytes, OpenCLMemoryType mem_type, cl::Buffer **result) {
    cl_mem_flags converted_type = -1;
    switch (mem_type) {
        case OpenCLMemoryType::ReadOnly: {
            converted_type = CL_MEM_READ_ONLY;
            break;
        }

        case OpenCLMemoryType::WriteOnly: {
            converted_type = CL_MEM_WRITE_ONLY;
            break;
        }

        case OpenCLMemoryType::ReadWrite: {
            converted_type = CL_MEM_READ_WRITE;
            break;
        }
    }

    cl_int err;
    *result = new cl::Buffer(global::openCLContext, converted_type, bytes, nullptr, &err);
    return get_opencl_error_code(err);
}

void opencl_free(cl::Buffer *buf) {
    delete buf;
}

OpenCLErrorCode opencl_write(const cl::Buffer &dst, const void *src, uint64_t bytes) {
    // We always block here, since we need the data to be available. At some point we might want to add the option
    // for a non-blocking write, but that's pretty complicated to handle correctly.
    return get_opencl_error_code(
            global::openCLQueue.enqueueWriteBuffer(dst, CL_TRUE, 0, bytes, src));
}

OpenCLErrorCode opencl_read(void *dst, const cl::Buffer &src, uint64_t bytes) {
    // We always block here, since we need the data to be available. At some point we might want to add the option
        // for a non-blocking read, but that's pretty complicated to handle correctly.
    return get_opencl_error_code(
            global::openCLQueue.enqueueReadBuffer(src, CL_TRUE, 0, bytes, dst));
}

OpenCLErrorCode opencl_copy(cl::Buffer &dst, const cl::Buffer &src, size_t bytes) {
    return get_opencl_error_code(global::openCLQueue.enqueueCopyBuffer(src, dst, 0, 0, bytes));
}

#ifdef __cplusplus
extern "C" {
#endif

OpenCLErrorCode opencl_allocate_ffi(uint64_t bytes, OpenCLMemoryType mem_type, void **ptr) {
    return opencl_allocate(bytes, mem_type, (cl::Buffer **) (ptr));
}

void opencl_free_ffi(void *buf) {
    delete (cl::Buffer *) buf;
}

OpenCLErrorCode opencl_write_ffi(void *dst, const void *src, uint64_t bytes) {
    return opencl_write(*(cl::Buffer *) dst, src, bytes);
}

OpenCLErrorCode opencl_read_ffi(void *dst, const void *src, uint64_t bytes) {
    return opencl_read(dst, *(cl::Buffer *) src, bytes);
}

OpenCLErrorCode opencl_copy_ffi(void *dst, const void *src, uint64_t bytes) {
    return opencl_copy(*(cl::Buffer *) dst, *(cl::Buffer *) src, bytes);
}

#ifdef __cplusplus
}
#endif

#else

void opencl_memory_placeholder() {}

#endif // HASTY_IMPL_HAS_OPENCL
