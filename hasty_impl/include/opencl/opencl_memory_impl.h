#ifndef HASTY_IMPL_OPENCL_MEMORY_IMPL_H
#define HASTY_IMPL_OPENCL_MEMORY_IMPL_H
#ifdef HASTY_IMPL_HAS_OPENCL

#include <stdbool.h>
#include <stdint.h>
#include "opencl_error_types.h"

#ifdef __cplusplus
extern "C" {
#endif

enum OpenCLMemoryType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
};

enum OpenCLMemCopyType {
    HostToDevice,
    DeviceToHost
};

enum OpenCLErrorCode opencl_allocate_voidptr(uint64_t bytes, enum OpenCLMemoryType mem_type, void **ptr);

void opencl_free_voidptr(void *buf);

enum OpenCLErrorCode
opencl_write_voidptr(void *dst, const void *src, uint64_t bytes, bool blocking);

enum OpenCLErrorCode
opencl_read_voidptr(void *dst, const void *src, uint64_t bytes, bool blocking);

#ifdef __cplusplus
}
#endif

#endif // HASTY_IMPL_HAS_OPENCL
#endif //HASTY_IMPL_OPENCL_MEMORY_IMPL_H
