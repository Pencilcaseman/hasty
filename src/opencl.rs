use crate::hasty_impl;

/// A collection of OpenCL error codes that can be returned.
/// Note that this enum contains the `Success` variant -- this
/// should be converted into `Result::Ok` or `Option::Some` when
/// returned. Other variants should be converted into `Result::Err`
/// or `Option::None` when returned.
#[derive(Debug)]
pub enum OpenCLErrorCode {
    /// The operation completed successfully
    Success,

    /// The specified device could not be found
    DeviceNotFound,

    /// The specified device is not available
    DeviceNotAvailable,

    /// The compiler is not available
    CompilerNotAvailable,

    /// Memory allocation failed
    AllocationFailure,

    /// The device is out of resources
    OutOfResources,

    /// The host is out of memory
    OutOfHostMemory,

    /// Profiling information is not available
    ProfilingInfoNotAvailable,

    /// Two memcopy operations overlapped
    MemoryCopyOverlap,

    /// The image format does not match the image format descriptor
    ImageFormatMismatch,

    /// The image format is not supported
    ImageFormatNotSupported,

    /// The program failed to build
    /// **Note**: Contains the build log
    BuildProgramFailure(String),

    /// Map operation failed
    MapFailure,

    /// An invalid value was passed as an argument
    InvalidValue,

    /// An invalid device type was passed
    InvalidDeviceType,

    /// The current platform is invalid
    InvalidPlatform,

    /// The specified device is invalid
    InvalidDevice,

    /// The internal context is invalid (please report this error)
    InvalidContext,

    /// The specified queue properties are invalid (please report this error)
    InvalidQueueProperties,

    /// The specified command queue is invalid (please report this error)
    InvalidCommandQueue,

    /// The specified host pointer is invalid (please report this error)
    InvalidHostPointer,

    /// The specified memory object is invalid
    InvalidMemoryObject,

    /// The image format descriptor is invalid
    InvalidImageFormatDescriptor,

    /// The image size is invalid
    InvalidImageSize,

    /// The binary is invalid
    InvalidBinary,

    /// The build options are invalid
    InvalidBuildOptions,

    /// The program is invalid
    InvalidProgram,

    /// The program executable is invalid
    InvalidProgramExecutable,

    /// The kernel name is invalid
    InvalidKernelName,

    /// The kernel definition is invalid
    InvalidKernelDefinition,

    /// The kernel is invalid
    InvalidKernel,

    /// An argument's index is invalid
    InvalidArgIndex,

    /// An argument's value is invalid
    InvalidArgValue,

    /// An argument's size is invalid
    InvalidArgSize,

    /// The kernel arguments are invalid
    InvalidKernelArgs,

    /// An invalid work dimension was passed
    InvalidWorkDimension,

    /// An invalid work group size was passed
    InvalidWorkGroupSize,

    /// A work item with an invalid size was passed
    InvalidWorkItemSize,

    /// An invalid global offset was used
    InvalidGlobalOffset,

    /// An invalid event wait list was used
    InvalidEventWaitList,

    /// An invalid event occurred
    InvalidEvent,

    /// An invalid operation was performed
    InvalidOperation,

    /// An invalid OpenGL object was used (please report this error)
    InvalidGLObject,

    /// The buffer size is invalid
    InvalidBufferSize,

    /// The mip level is invalid
    InvalidMipLevel,

    /// The global work size is invalid
    InvalidGlobalWorkSize,

    /// An unknown error occurred. This is a catch-all error and should
    /// never be returned. If you see this, please double-check your code
    /// and create an issue on the GitHub repository.
    UnknownError,

    /// The requested routine is not implemented
    RoutineNotImplemented,

    /// An invalid matrix was passed.
    /// **Note**: Contains the invalid matrix (A, B, or C)
    InvalidMatrix(char),

    /// An invalid vector was passed.
    /// **Note**: Contains the invalid vector (X or Y)
    InvalidVector(char),

    /// An invalid dimension was passed
    InvalidDimension,

    /// An invalid leading dimension was passed.
    /// **Note**: Contains the invalid leading dimension (A, B, or C)
    InvalidLD(char),

    /// An invalid vector  increment was passed.
    /// **Note**: Contains the invalid increment (X or Y)
    InvalidInc(char),

    /// An invalid matrix buffer size was passed.
    /// **Note**: Contains the invalid matrix buffer size (A, B, or C)
    InvalidMatrixBufferSize(char),

    /// An invalid vector buffer size was passed.
    /// **Note**: Contains the invalid vector buffer size (X or Y)
    InvalidVectorBufferSize(char),

    /// The temporary GEMM buffer is too small. This is an internal error
    TempGemmBufferTooSmall,

    /// Negative batch count was passed. Batch count must be positive
    BatchCountMustBePositive,

    /// Trying to override parameters for an invalid kernel
    TryingToOverrideParametersForInvalidKernel,

    /// Missing override parameters
    MissingOverrideParameters,

    /// The device is out of memory
    DeviceOutOfMemory,

    /// Half precision is not supported on the device
    HalfPrecisionNotSupported,

    /// The unit-sized vector is invalid
    UnitSizedVectorInvalid,

    /// The unit-sized vector is too small
    UnitSizedVectorTooSmall,

    /// The device entry was not found
    DeviceEntryNotFound,
}

/// Memory configuration options
pub enum OpenCLMemoryType {
    /// Memory can only be read from. Writing to this memory is undefined behaviour
    ReadOnly,

    /// Memory can only be written to. Reading from this memory is undefined behaviour
    WriteOnly,

    /// Memory can be read from or written to
    ReadWrite,
}

/// Data transfer modes
pub enum OpenCLMemCopyType {
    /// Copy from host memory to device memory
    HostToDevice,

    /// Copy from device memory to host memory
    DeviceToHost,
}

impl OpenCLErrorCode {
    /// Convert from the FFI error code to the Rust error code
    pub unsafe fn from_ffi(code: hasty_impl::OpenCLErrorCode) -> Self {
        match code {
            hasty_impl::OpenCLErrorCode_Success => OpenCLErrorCode::Success,
            hasty_impl::OpenCLErrorCode_DeviceNotFound => OpenCLErrorCode::DeviceNotFound,
            hasty_impl::OpenCLErrorCode_DeviceNotAvailable => OpenCLErrorCode::DeviceNotAvailable,
            hasty_impl::OpenCLErrorCode_CompilerNotAvailable => OpenCLErrorCode::CompilerNotAvailable,
            hasty_impl::OpenCLErrorCode_MemoryObjectAllocationFailure => OpenCLErrorCode::AllocationFailure,
            hasty_impl::OpenCLErrorCode_OutOfResources => OpenCLErrorCode::OutOfResources,
            hasty_impl::OpenCLErrorCode_OutOfHostMemory => OpenCLErrorCode::OutOfHostMemory,
            hasty_impl::OpenCLErrorCode_ProfilingInfoNotAvailable => OpenCLErrorCode::ProfilingInfoNotAvailable,
            hasty_impl::OpenCLErrorCode_MemoryCopyOverlap => OpenCLErrorCode::MemoryCopyOverlap,
            hasty_impl::OpenCLErrorCode_ImageFormatMismatch => OpenCLErrorCode::ImageFormatMismatch,
            hasty_impl::OpenCLErrorCode_ImageFormatNotSupported => OpenCLErrorCode::ImageFormatNotSupported,
            hasty_impl::OpenCLErrorCode_BuildProgramFailure => OpenCLErrorCode::BuildProgramFailure(String::from("")),
            hasty_impl::OpenCLErrorCode_MapFailure => OpenCLErrorCode::MapFailure,
            hasty_impl::OpenCLErrorCode_InvalidValue => OpenCLErrorCode::InvalidValue,
            hasty_impl::OpenCLErrorCode_InvalidDeviceType => OpenCLErrorCode::InvalidDeviceType,
            hasty_impl::OpenCLErrorCode_InvalidPlatform => OpenCLErrorCode::InvalidPlatform,
            hasty_impl::OpenCLErrorCode_InvalidDevice => OpenCLErrorCode::InvalidDevice,
            hasty_impl::OpenCLErrorCode_InvalidContext => OpenCLErrorCode::InvalidContext,
            hasty_impl::OpenCLErrorCode_InvalidQueueProperties => OpenCLErrorCode::InvalidQueueProperties,
            hasty_impl::OpenCLErrorCode_InvalidCommandQueue => OpenCLErrorCode::InvalidCommandQueue,
            hasty_impl::OpenCLErrorCode_InvalidHostPointer => OpenCLErrorCode::InvalidHostPointer,
            hasty_impl::OpenCLErrorCode_InvalidMemoryObject => OpenCLErrorCode::InvalidMemoryObject,
            hasty_impl::OpenCLErrorCode_InvalidImageFormatDescriptor => OpenCLErrorCode::InvalidImageFormatDescriptor,
            hasty_impl::OpenCLErrorCode_InvalidImageSize => OpenCLErrorCode::InvalidImageSize,
            hasty_impl::OpenCLErrorCode_InvalidSampler => OpenCLErrorCode::InvalidValue,
            hasty_impl::OpenCLErrorCode_InvalidBinary => OpenCLErrorCode::InvalidBinary,
            hasty_impl::OpenCLErrorCode_InvalidBuildOptions => OpenCLErrorCode::InvalidBuildOptions,
            hasty_impl::OpenCLErrorCode_InvalidProgram => OpenCLErrorCode::InvalidProgram,
            hasty_impl::OpenCLErrorCode_InvalidProgramExecutable => OpenCLErrorCode::InvalidProgramExecutable,
            hasty_impl::OpenCLErrorCode_InvalidKernelName => OpenCLErrorCode::InvalidKernelName,
            hasty_impl::OpenCLErrorCode_InvalidKernelDefinition => OpenCLErrorCode::InvalidKernelDefinition,
            hasty_impl::OpenCLErrorCode_InvalidKernel => OpenCLErrorCode::InvalidKernel,
            hasty_impl::OpenCLErrorCode_InvalidArgIndex => OpenCLErrorCode::InvalidArgIndex,
            hasty_impl::OpenCLErrorCode_InvalidArgValue => OpenCLErrorCode::InvalidArgValue,
            hasty_impl::OpenCLErrorCode_InvalidArgSize => OpenCLErrorCode::InvalidArgSize,
            hasty_impl::OpenCLErrorCode_InvalidKernelArgs => OpenCLErrorCode::InvalidKernelArgs,
            hasty_impl::OpenCLErrorCode_InvalidWorkDimension => OpenCLErrorCode::InvalidWorkDimension,
            hasty_impl::OpenCLErrorCode_InvalidWorkGroupSize => OpenCLErrorCode::InvalidWorkGroupSize,
            hasty_impl::OpenCLErrorCode_InvalidWorkItemSize => OpenCLErrorCode::InvalidWorkItemSize,
            hasty_impl::OpenCLErrorCode_InvalidGlobalOffset => OpenCLErrorCode::InvalidGlobalOffset,
            hasty_impl::OpenCLErrorCode_InvalidEventWaitList => OpenCLErrorCode::InvalidEventWaitList,
            hasty_impl::OpenCLErrorCode_InvalidEvent => OpenCLErrorCode::InvalidEvent,
            hasty_impl::OpenCLErrorCode_InvalidOperation => OpenCLErrorCode::InvalidOperation,
            hasty_impl::OpenCLErrorCode_InvalidGLObject => OpenCLErrorCode::InvalidGLObject,
            hasty_impl::OpenCLErrorCode_InvalidBufferSize => OpenCLErrorCode::InvalidBufferSize,
            hasty_impl::OpenCLErrorCode_InvalidMipLevel => OpenCLErrorCode::InvalidMipLevel,
            hasty_impl::OpenCLErrorCode_InvalidGlobalWorkSize => OpenCLErrorCode::InvalidGlobalWorkSize,

            hasty_impl::OpenCLErrorCode_UnknownError => OpenCLErrorCode::UnknownError,

            hasty_impl::OpenCLErrorCode_RoutineNotImplemented => OpenCLErrorCode::RoutineNotImplemented,
            hasty_impl::OpenCLErrorCode_InvalidMatrixA => OpenCLErrorCode::InvalidMatrix('A'),
            hasty_impl::OpenCLErrorCode_InvalidMatrixB => OpenCLErrorCode::InvalidMatrix('B'),
            hasty_impl::OpenCLErrorCode_InvalidMatrixC => OpenCLErrorCode::InvalidMatrix('C'),
            hasty_impl::OpenCLErrorCode_InvalidVectorX => OpenCLErrorCode::InvalidVector('X'),
            hasty_impl::OpenCLErrorCode_InvalidVectorY => OpenCLErrorCode::InvalidVector('Y'),
            hasty_impl::OpenCLErrorCode_InvalidDimension => OpenCLErrorCode::InvalidDimension,
            hasty_impl::OpenCLErrorCode_InvalidLDA => OpenCLErrorCode::InvalidLD('A'),
            hasty_impl::OpenCLErrorCode_InvalidLDB => OpenCLErrorCode::InvalidLD('B'),
            hasty_impl::OpenCLErrorCode_InvalidLDC => OpenCLErrorCode::InvalidLD('C'),
            hasty_impl::OpenCLErrorCode_InvalidIncX => OpenCLErrorCode::InvalidInc('X'),
            hasty_impl::OpenCLErrorCode_InvalidIncY => OpenCLErrorCode::InvalidInc('Y'),
            hasty_impl::OpenCLErrorCode_InvalidMatrixBufferSizeA => OpenCLErrorCode::InvalidMatrixBufferSize('A'),
            hasty_impl::OpenCLErrorCode_InvalidMatrixBufferSizeB => OpenCLErrorCode::InvalidMatrixBufferSize('B'),
            hasty_impl::OpenCLErrorCode_InvalidMatrixBufferSizeC => OpenCLErrorCode::InvalidMatrixBufferSize('C'),
            hasty_impl::OpenCLErrorCode_InvalidVectorBufferSizeX => OpenCLErrorCode::InvalidVectorBufferSize('X'),
            hasty_impl::OpenCLErrorCode_InvalidVectorBufferSizeY => OpenCLErrorCode::InvalidVectorBufferSize('Y'),
            hasty_impl::OpenCLErrorCode_TemporaryGEMMBufferTooSmall => OpenCLErrorCode::TempGemmBufferTooSmall,
            hasty_impl::OpenCLErrorCode_BatchCountMustBePositive => OpenCLErrorCode::BatchCountMustBePositive,
            hasty_impl::OpenCLErrorCode_TryingToOverrideParametersForInvalidKernel => OpenCLErrorCode::TryingToOverrideParametersForInvalidKernel,
            hasty_impl::OpenCLErrorCode_MissingOverrideParameters => OpenCLErrorCode::MissingOverrideParameters,
            hasty_impl::OpenCLErrorCode_DeviceOutOfMemory => OpenCLErrorCode::DeviceOutOfMemory,
            hasty_impl::OpenCLErrorCode_HalfPrecisionNotSupported => OpenCLErrorCode::HalfPrecisionNotSupported,
            hasty_impl::OpenCLErrorCode_UnitSizedVectorInvalid => OpenCLErrorCode::UnitSizedVectorInvalid,
            hasty_impl::OpenCLErrorCode_UnitSizedVectorTooSmall => OpenCLErrorCode::UnitSizedVectorTooSmall,
            hasty_impl::OpenCLErrorCode_DeviceEntryNotFound => OpenCLErrorCode::DeviceEntryNotFound,
            _ => OpenCLErrorCode::UnknownError
        }
    }
}

impl OpenCLMemoryType {
    /// Convert from the Rust memory type to the FFI memory type
    pub unsafe fn to_ffi(&self) -> hasty_impl::OpenCLMemoryType {
        match self {
            OpenCLMemoryType::ReadOnly => hasty_impl::OpenCLMemoryType_ReadOnly,
            OpenCLMemoryType::WriteOnly => hasty_impl::OpenCLMemoryType_WriteOnly,
            OpenCLMemoryType::ReadWrite => hasty_impl::OpenCLMemoryType_ReadWrite,
        }
    }

    /// Convert from the FFI memory type to the Rust memory type
    pub unsafe fn from_ffi(mem_type: hasty_impl::OpenCLMemoryType) -> Self {
        match mem_type {
            hasty_impl::OpenCLMemoryType_ReadOnly => OpenCLMemoryType::ReadOnly,
            hasty_impl::OpenCLMemoryType_WriteOnly => OpenCLMemoryType::WriteOnly,
            hasty_impl::OpenCLMemoryType_ReadWrite => OpenCLMemoryType::ReadWrite,
            _ => panic!("Invalid memory type"),
        }
    }
}

/// Configure OpenCL -- Temporary function. This will be replaced soon
pub unsafe fn configure_opencl() {
    unsafe {
        hasty_impl::configureOpenCL();
    }
}

/// Allocate memory on the OpenCL device
pub unsafe fn opencl_allocate(bytes: usize, mem_type: OpenCLMemoryType) -> Result<*mut ::std::os::raw::c_void, OpenCLErrorCode> {
    let mut buffer: *mut ::std::os::raw::c_void = ::std::ptr::null_mut();

    let ret = hasty_impl::opencl_allocate_voidptr(bytes as u64, mem_type.to_ffi(), &mut buffer);

    if ret == hasty_impl::OpenCLErrorCode_Success {
        Ok(buffer)
    } else {
        Err(OpenCLErrorCode::from_ffi(ret))
    }
}

/// Free memory on the OpenCL device
///
/// **Note**: This function does not return an error code. If the memory
/// could not be freed, it will panic (or segfault...). As a result, the function
/// signature is `unsafe` -- the caller must ensure that the memory can be freed.
pub unsafe fn opencl_free(buffer: *mut ::std::os::raw::c_void) {
    hasty_impl::opencl_free_voidptr(buffer);
}

/// Write data to the OpenCL device
///
/// **Note**: There are no checks on the pointers, nor the size of the data. The caller
/// must ensure that everything is valid.
pub unsafe fn opencl_write(dst: *mut ::std::os::raw::c_void, src: *const ::std::os::raw::c_void, bytes: usize) -> Result<(), OpenCLErrorCode> {
    let ret = hasty_impl::opencl_write_voidptr(dst, src, bytes as u64, true);

    if ret == hasty_impl::OpenCLErrorCode_Success {
        Ok(())
    } else {
        Err(OpenCLErrorCode::from_ffi(ret))
    }
}

/// Read data from the OpenCL device
///
/// **Note**: There are no checks on the pointers, nor the size of the data. The caller
/// must ensure that everything is valid.
pub unsafe fn opencl_read(dst: *mut ::std::os::raw::c_void, src: *const ::std::os::raw::c_void, bytes: usize) -> Result<(), OpenCLErrorCode> {
    let ret = hasty_impl::opencl_read_voidptr(dst, src, bytes as u64, true);

    if ret == hasty_impl::OpenCLErrorCode_Success {
        Ok(())
    } else {
        Err(OpenCLErrorCode::from_ffi(ret))
    }
}
