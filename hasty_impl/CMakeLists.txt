cmake_minimum_required(VERSION 3.16)
project(hasty_impl LANGUAGES C CXX)
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

set(CMAKE_CXX_STANDARD 17)

include(FetchContent)

file(WRITE ${CMAKE_BINARY_DIR}/library_config.txt "")

macro(write_library_config data)
    file(APPEND ${CMAKE_BINARY_DIR}/library_config.txt "${data}\n")
endmacro()

file(GLOB_RECURSE HASTY_IMPL_HEADER
    "${CMAKE_CURRENT_SOURCE_DIR}/include/*.h"
    "${CMAKE_CURRENT_SOURCE_DIR}/include/*.hpp"
)

file(GLOB_RECURSE HASTY_IMPL_SOURCE
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp"
)

add_library(hasty_impl STATIC ${HASTY_IMPL_SOURCE})

target_include_directories(hasty_impl PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Disable shared libraries, since they tend to cause problems
set(BUILD_SHARED_LIBS OFF CACHE BOOL "Build shared libraries" FORCE)

set(IS_LINUX OFF)
set(IS_MACOS OFF)
set(IS_WINDOWS OFF)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    set(IS_LINUX ON)
endif()

if(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    set(IS_MACOS ON)
endif()

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    set(IS_WINDOWS ON)
endif()

if(HASTY_USE_BLAS)
    target_compile_definitions(hasty_impl PUBLIC HASTY_IMPL_HAS_BLAS)
    include(blas_config)
    configure_blas()
endif()

if (HASTY_USE_OPENCL)
    find_package(OpenCL REQUIRED)

    message(STATUS "[ HASTY_IMPL ] Using OpenCL ${OpenCL_VERSION_STRING}")
    target_include_directories(hasty_impl PUBLIC ${OpenCL_INCLUDE_DIRS})
    target_link_libraries(hasty_impl PUBLIC ${OpenCL_LIBRARIES})
    target_compile_definitions(hasty_impl PUBLIC HASTY_IMPL_HAS_OPENCL)

    # Build static CLBlast library
    set(BUILD_SHARED_LIBS OFF)
    add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/vendor/CLBlast")
    target_link_libraries(hasty_impl PUBLIC clblast)
    target_include_directories(hasty_impl PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/vendor/CLBlast/include")

    if (IS_MACOS)
        # Add -framework OpenCL to the linker flags
        write_library_config("opencl")
        target_link_libraries(hasty_impl PUBLIC "-framework OpenCL")

        # Define CL_HPP_TARGET_OPENCL_VERSION to the version of OpenCL being used
        message(STATUS "[ HASTY_IMPL ] OpenCL Version: ${OpenCL_VERSION_MAJOR}${OpenCL_VERSION_MINOR}0")
        set(OpenCLVersion ${OpenCL_VERSION_MAJOR}${OpenCL_VERSION_MINOR}0)
        target_compile_definitions(hasty_impl PUBLIC CL_HPP_TARGET_OPENCL_VERSION=${OpenCLVersion})
        target_compile_definitions(hasty_impl PUBLIC CL_HPP_MINIMUM_OPENCL_VERSION=${OpenCLVersion})

        if (NOT CMAKE_CXX_COMPILER_ID MATCHES "AppleClang")
            message(WARNING "[ HASTY_IMPL ] Accelerate OpenCL is designed for AppleClang. Relaxing some conditions")
            target_compile_options(hasty_impl PUBLIC "-flax-vector-conversions")
        endif ()
    endif ()
endif ()

# Install logic
install(TARGETS hasty_impl DESTINATION lib)
install(FILES ${HASTY_IMPL_HEADER} DESTINATION include)
