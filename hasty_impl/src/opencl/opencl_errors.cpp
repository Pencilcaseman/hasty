#ifdef HASTY_IMPL_HAS_OPENCL

#include "opencl/opencl.hpp"
#include "clblast.h"
#include "opencl/opencl_errors.hpp"
#include <map>

//const char *getOpenCLErrorString(int64_t error) {
//    static const char *strings[] = {            // Error Codes
//            "CL_SUCCESS",                       //   0
//            "CL_DEVICE_NOT_FOUND",              //  -1
//            "CL_DEVICE_NOT_AVAILABLE",          //  -2
//            "CL_COMPILER_NOT_AVAILABLE",        //  -3
//            "CL_MEM_OBJECT_ALLOCATION_FAILURE", //  -4
//            "CL_OUT_OF_RESOURCES",              //  -5
//            "CL_OUT_OF_HOST_MEMORY",            //  -6
//            "CL_PROFILING_INFO_NOT_AVAILABLE",  //  -7
//            "CL_MEM_COPY_OVERLAP",              //  -8
//            "CL_IMAGE_FORMAT_MISMATCH",         //  -9
//            "CL_IMAGE_FORMAT_NOT_SUPPORTED",    //  -10
//            "CL_BUILD_PROGRAM_FAILURE",         //  -11
//            "CL_MAP_FAILURE",                   //  -12
//
//            "", //  -13
//            "", //  -14
//            "", //  -15
//            "", //  -16
//            "", //  -17
//            "", //  -18
//            "", //  -19
//
//            "", //  -20
//            "", //  -21
//            "", //  -22
//            "", //  -23
//            "", //  -24
//            "", //  -25
//            "", //  -26
//            "", //  -27
//            "", //  -28
//            "", //  -29
//
//            "CL_INVALID_VALUE",                   //  -30
//            "CL_INVALID_DEVICE_TYPE",             //  -31
//            "CL_INVALID_PLATFORM",                //  -32
//            "CL_INVALID_DEVICE",                  //  -33
//            "CL_INVALID_CONTEXT",                 //  -34
//            "CL_INVALID_QUEUE_PROPERTIES",        //  -35
//            "CL_INVALID_COMMAND_QUEUE",           //  -36
//            "CL_INVALID_HOST_PTR",                //  -37
//            "CL_INVALID_MEM_OBJECT",              //  -38
//            "CL_INVALID_IMAGE_FORMAT_DESCRIPTOR", //  -39
//            "CL_INVALID_IMAGE_SIZE",              //  -40
//            "CL_INVALID_SAMPLER",                 //  -41
//            "CL_INVALID_BINARY",                  //  -42
//            "CL_INVALID_BUILD_OPTIONS",           //  -43
//            "CL_INVALID_PROGRAM",                 //  -44
//            "CL_INVALID_PROGRAM_EXECUTABLE",      //  -45
//            "CL_INVALID_KERNEL_NAME",             //  -46
//            "CL_INVALID_KERNEL_DEFINITION",       //  -47
//            "CL_INVALID_KERNEL",                  //  -48
//            "CL_INVALID_ARG_INDEX",               //  -49
//            "CL_INVALID_ARG_VALUE",               //  -50
//            "CL_INVALID_ARG_SIZE",                //  -51
//            "CL_INVALID_KERNEL_ARGS",             //  -52
//            "CL_INVALID_WORK_DIMENSION",          //  -53
//            "CL_INVALID_WORK_GROUP_SIZE",         //  -54
//            "CL_INVALID_WORK_ITEM_SIZE",          //  -55
//            "CL_INVALID_GLOBAL_OFFSET",           //  -56
//            "CL_INVALID_EVENT_WAIT_LIST",         //  -57
//            "CL_INVALID_EVENT",                   //  -58
//            "CL_INVALID_OPERATION",               //  -59
//            "CL_INVALID_GL_OBJECT",               //  -60
//            "CL_INVALID_BUFFER_SIZE",             //  -61
//            "CL_INVALID_MIP_LEVEL",               //  -62
//            "CL_INVALID_GLOBAL_WORK_SIZE",        //  -63
//            "CL_UNKNOWN_ERROR_CODE"};
//
//    if (error >= -63 && error <= 0)
//        return strings[-error];
//    else
//        return strings[64];
//}

OpenCLErrorCode get_opencl_error_code(int64_t error) {
    // Error Codes
    static OpenCLErrorCode codes[] = {
            OpenCLErrorCode::Success,                        //   0
            OpenCLErrorCode::DeviceNotFound,                 //  -1
            OpenCLErrorCode::DeviceNotAvailable,             //  -2
            OpenCLErrorCode::CompilerNotAvailable,           //  -3
            OpenCLErrorCode::MemoryObjectAllocationFailure,  //  -4
            OpenCLErrorCode::OutOfResources,                 //  -5
            OpenCLErrorCode::OutOfHostMemory,                //  -6
            OpenCLErrorCode::ProfilingInfoNotAvailable,      //  -7
            OpenCLErrorCode::MemoryCopyOverlap,              //  -8
            OpenCLErrorCode::ImageFormatMismatch,            //  -9
            OpenCLErrorCode::ImageFormatNotSupported,        //  -10
            OpenCLErrorCode::BuildProgramFailure,            //  -11
            OpenCLErrorCode::MapFailure,                     //  -12

            OpenCLErrorCode::UnknownError, //  -13
            OpenCLErrorCode::UnknownError, //  -14
            OpenCLErrorCode::UnknownError, //  -15
            OpenCLErrorCode::UnknownError, //  -16
            OpenCLErrorCode::UnknownError, //  -17
            OpenCLErrorCode::UnknownError, //  -18
            OpenCLErrorCode::UnknownError, //  -19

            OpenCLErrorCode::UnknownError, //  -20
            OpenCLErrorCode::UnknownError, //  -21
            OpenCLErrorCode::UnknownError, //  -22
            OpenCLErrorCode::UnknownError, //  -23
            OpenCLErrorCode::UnknownError, //  -24
            OpenCLErrorCode::UnknownError, //  -25
            OpenCLErrorCode::UnknownError, //  -26
            OpenCLErrorCode::UnknownError, //  -27
            OpenCLErrorCode::UnknownError, //  -28
            OpenCLErrorCode::UnknownError, //  -29

            InvalidValue,                      //  -30
            InvalidDeviceType,                 //  -31
            InvalidPlatform,                   //  -32
            InvalidDevice,                     //  -33
            InvalidContext,                    //  -34
            InvalidQueueProperties,            //  -35
            InvalidCommandQueue,               //  -36
            InvalidHostPointer,                //  -37
            InvalidMemoryObject,               //  -38
            InvalidImageFormatDescriptor,      //  -39
            InvalidImageSize,                  //  -40
            InvalidSampler,                    //  -41
            InvalidBinary,                     //  -42
            InvalidBuildOptions,               //  -43
            InvalidProgram,                    //  -44
            InvalidProgramExecutable,          //  -45
            InvalidKernelName,                 //  -46
            InvalidKernelDefinition,           //  -47
            InvalidKernel,                     //  -48
            InvalidArgIndex,                   //  -49
            InvalidArgValue,                   //  -50
            InvalidArgSize,                    //  -51
            InvalidKernelArgs,                 //  -52
            InvalidWorkDimension,              //  -53
            InvalidWorkGroupSize,              //  -54
            InvalidWorkItemSize,               //  -55
            InvalidGlobalOffset,               //  -56
            InvalidEventWaitList,              //  -57
            InvalidEvent,                      //  -58
            InvalidOperation,                  //  -59
            InvalidGLObject,                   //  -60
            InvalidBufferSize,                 //  -61
            InvalidMipLevel,                   //  -62
            InvalidGlobalWorkSize,             //  -63
            UnknownError};

    if (error >= -63 && error <= 0)
        return codes[-error];
    else
        return codes[64];
}

OpenCLErrorCode getCLBlastErrorCode(clblast::StatusCode status) {
    // clang-format off
    static const std::map<clblast::StatusCode, OpenCLErrorCode> statusMap = {
            {clblast::StatusCode::kSuccess,                    OpenCLErrorCode::Success},
            {clblast::StatusCode::kOpenCLCompilerNotAvailable, OpenCLErrorCode::CompilerNotAvailable},
            {clblast::StatusCode::kTempBufferAllocFailure,     OpenCLErrorCode::MemoryObjectAllocationFailure},
            {clblast::StatusCode::kOpenCLOutOfResources,       OpenCLErrorCode::OutOfResources},
            {clblast::StatusCode::kOpenCLOutOfHostMemory,      OpenCLErrorCode::OutOfHostMemory},
            {clblast::StatusCode::kOpenCLBuildProgramFailure,  OpenCLErrorCode::BuildProgramFailure},
            {clblast::StatusCode::kInvalidValue,               OpenCLErrorCode::InvalidValue},
            {clblast::StatusCode::kInvalidCommandQueue,        OpenCLErrorCode::InvalidCommandQueue},
            {clblast::StatusCode::kInvalidMemObject,           OpenCLErrorCode::InvalidMemoryObject},
            {clblast::StatusCode::kInvalidBinary,              OpenCLErrorCode::InvalidBinary},
            {clblast::StatusCode::kInvalidBuildOptions,        OpenCLErrorCode::InvalidBuildOptions},
            {clblast::StatusCode::kInvalidProgram,             OpenCLErrorCode::InvalidProgram},
            {clblast::StatusCode::kInvalidProgramExecutable,   OpenCLErrorCode::InvalidProgramExecutable},
            {clblast::StatusCode::kInvalidKernelName,          OpenCLErrorCode::InvalidKernelName},
            {clblast::StatusCode::kInvalidKernelDefinition,    OpenCLErrorCode::InvalidKernelDefinition},
            {clblast::StatusCode::kInvalidKernel,              OpenCLErrorCode::InvalidKernel},
            {clblast::StatusCode::kInvalidArgIndex,            OpenCLErrorCode::InvalidArgIndex},
            {clblast::StatusCode::kInvalidArgValue,            OpenCLErrorCode::InvalidArgValue},
            {clblast::StatusCode::kInvalidArgSize,             OpenCLErrorCode::InvalidArgSize},
            {clblast::StatusCode::kInvalidKernelArgs,          OpenCLErrorCode::InvalidKernelArgs},
            {clblast::StatusCode::kInvalidLocalNumDimensions,  OpenCLErrorCode::InvalidWorkDimension},
            {clblast::StatusCode::kInvalidLocalThreadsTotal,   OpenCLErrorCode::InvalidWorkGroupSize},
            {clblast::StatusCode::kInvalidLocalThreadsDim,     OpenCLErrorCode::InvalidWorkItemSize},
            {clblast::StatusCode::kInvalidGlobalOffset,        OpenCLErrorCode::InvalidGlobalOffset},
            {clblast::StatusCode::kInvalidEventWaitList,       OpenCLErrorCode::InvalidEventWaitList},
            {clblast::StatusCode::kInvalidEvent,               OpenCLErrorCode::InvalidEvent},
            {clblast::StatusCode::kInvalidOperation,           OpenCLErrorCode::InvalidOperation},
            {clblast::StatusCode::kInvalidBufferSize,          OpenCLErrorCode::InvalidBufferSize},
            {clblast::StatusCode::kInvalidGlobalWorkSize,      OpenCLErrorCode::InvalidGlobalWorkSize},
            {clblast::StatusCode::kNotImplemented,             OpenCLErrorCode::RoutineNotImplemented},
            {clblast::StatusCode::kInvalidMatrixA,             OpenCLErrorCode::InvalidMatrixA},
            {clblast::StatusCode::kInvalidMatrixB,             OpenCLErrorCode::InvalidMatrixB},
            {clblast::StatusCode::kInvalidMatrixC,             OpenCLErrorCode::InvalidMatrixC},
            {clblast::StatusCode::kInvalidVectorX,             OpenCLErrorCode::InvalidVectorX},
            {clblast::StatusCode::kInvalidVectorY,             OpenCLErrorCode::InvalidVectorY},
            {clblast::StatusCode::kInvalidDimension,           OpenCLErrorCode::InvalidDimension},
            {clblast::StatusCode::kInvalidLeadDimA,            OpenCLErrorCode::InvalidLDA},
            {clblast::StatusCode::kInvalidLeadDimB,            OpenCLErrorCode::InvalidLDB},
            {clblast::StatusCode::kInvalidLeadDimC,            OpenCLErrorCode::InvalidLDC},
            {clblast::StatusCode::kInvalidIncrementX,          OpenCLErrorCode::InvalidIncX},
            {clblast::StatusCode::kInvalidIncrementY,          OpenCLErrorCode::InvalidIncY},
            {clblast::StatusCode::kInsufficientMemoryA,        OpenCLErrorCode::InvalidMatrixBufferSizeA},
            {clblast::StatusCode::kInsufficientMemoryB,        OpenCLErrorCode::InvalidMatrixBufferSizeB},
            {clblast::StatusCode::kInsufficientMemoryC,        OpenCLErrorCode::InvalidMatrixBufferSizeC},
            {clblast::StatusCode::kInsufficientMemoryX,        OpenCLErrorCode::InvalidVectorBufferSizeX},
            {clblast::StatusCode::kInsufficientMemoryY,        OpenCLErrorCode::InvalidVectorBufferSizeY},
            {clblast::StatusCode::kInsufficientMemoryTemp,     OpenCLErrorCode::TemporaryGEMMBufferTooSmall},
            {clblast::StatusCode::kInvalidBatchCount,          OpenCLErrorCode::BatchCountMustBePositive},
            {clblast::StatusCode::kInsufficientMemoryTemp,     OpenCLErrorCode::TemporaryGEMMBufferTooSmall},
            {clblast::StatusCode::kInvalidBatchCount,          OpenCLErrorCode::BatchCountMustBePositive},
            {clblast::StatusCode::kInvalidOverrideKernel,      OpenCLErrorCode::TryingToOverrideParametersForInvalidKernel},
            {clblast::StatusCode::kMissingOverrideParameter,   OpenCLErrorCode::MissingOverrideParameters},
            {clblast::StatusCode::kInvalidLocalMemUsage,       OpenCLErrorCode::DeviceOutOfMemory},
            {clblast::StatusCode::kNoHalfPrecision,            OpenCLErrorCode::HalfPrecisionNotSupported},
            {clblast::StatusCode::kNoDoublePrecision,          OpenCLErrorCode::DoublePrecisionNotSupported},
            {clblast::StatusCode::kInvalidVectorScalar,        OpenCLErrorCode::UnitSizedVectorInvalid},
            {clblast::StatusCode::kInsufficientMemoryScalar,   OpenCLErrorCode::UnitSizedVectorTooSmall},
            {clblast::StatusCode::kDatabaseError,              OpenCLErrorCode::DeviceEntryNotFound},
            {clblast::StatusCode::kUnknownError,               OpenCLErrorCode::UnknownError},
            {clblast::StatusCode::kUnexpectedError,            OpenCLErrorCode::UnknownError}};
    // clang-format on

    auto it = statusMap.find(status);
    if (it != statusMap.end())
        return it->second;
    else
        return OpenCLErrorCode::UnknownError;
}

#else

void opencl_error_placeholder() {}

#endif // HASTY_IMPL_HAS_OPENCL