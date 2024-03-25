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

OpenCLErrorCode opencl_write(const cl::Buffer &dst, const void *src, uint64_t bytes, bool blocking) {
    return get_opencl_error_code(
            global::openCLQueue.enqueueWriteBuffer(dst, blocking ? CL_TRUE : CL_FALSE, 0, bytes, src));
}

OpenCLErrorCode opencl_read(void *dst, const cl::Buffer &src, uint64_t bytes, bool blocking) {
    return get_opencl_error_code(
            global::openCLQueue.enqueueReadBuffer(src, blocking ? CL_TRUE : CL_FALSE, 0, bytes, dst));
}

#ifdef __cplusplus
extern "C" {
#endif

OpenCLErrorCode opencl_allocate_voidptr(uint64_t bytes, OpenCLMemoryType mem_type, void **ptr) {
    return opencl_allocate(bytes, mem_type, (cl::Buffer **) (ptr));
}

void opencl_free_voidptr(void *buf) {
    delete (cl::Buffer *) buf;
}

OpenCLErrorCode opencl_write_voidptr(void *dst, const void *src, uint64_t bytes, bool blocking) {
    return opencl_write(*(cl::Buffer *) dst, src, bytes, blocking);
}

OpenCLErrorCode opencl_read_voidptr(void *dst, const void *src, uint64_t bytes, bool blocking) {
    return opencl_read(dst, *(cl::Buffer *) src, bytes, blocking);
}

#ifdef __cplusplus
}
#endif

#else

void opencl_memory_placeholder() {}

#endif // HASTY_IMPL_HAS_OPENCL
