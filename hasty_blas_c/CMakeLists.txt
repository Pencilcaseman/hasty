cmake_minimum_required(VERSION 3.16)
project(hasty_blas_c LANGUAGES C CXX)
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

include(FetchContent)

file(GLOB_RECURSE HASTY_BLAS_HEADER
    "${CMAKE_CURRENT_SOURCE_DIR}/include/*.h"
)

file(GLOB_RECURSE HASTY_BLAS_SOURCE
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.c"
)

add_library(hasty_blas_c STATIC ${HASTY_BLAS_SOURCE})

target_include_directories(hasty_blas_c PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Disable shared libraries, since they tend to cause problems
set(BUILD_SHARED_LIBS OFF CACHE BOOL "Build shared libraries" FORCE)

set(IS_LINUX OFF)
set(IS_MACOS OFF)
set(IS_WINDOWS OFF)

if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    set(IS_LINUX ON)
endif ()

if (CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    set(IS_MACOS ON)
endif ()

if (CMAKE_SYSTEM_NAME STREQUAL "Windows")
    set(IS_WINDOWS ON)
endif ()

include(blas_config)
configure_blas()

# Install logic
install(TARGETS hasty_blas_c DESTINATION lib)
install(FILES ${HASTY_BLAS_HEADER} DESTINATION include)
