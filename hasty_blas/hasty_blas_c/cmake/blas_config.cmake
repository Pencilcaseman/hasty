# Identify which BLAS library is being used based on the filename or path
macro(identify_blas filename)
    if ("${filename}" MATCHES "(openblas|/openblas/).*")
        set(BLAS_LIB "OPENBLAS")
    elseif ("${filename}" MATCHES "(mkl|/mkl/).*")
        set(BLAS_LIB "MKLBLAS")
    elseif ("${filename}" MATCHES "(atlas|/atlas/).*")
        set(BLAS_LIB "ATLAS")
    elseif ("${filename}" MATCHES "Accelerate\\.framework")
        set(BLAS_LIB "ACCELERATE")
    else ()
        set(BLAS_LIB "GENERIC")
    endif ()

    message(STATUS "[ HASTY_BLAS_C ] Identified BLAS Library: ${BLAS_LIB}")
endmacro()

macro(write_blas_config data)
    file(WRITE ${CMAKE_BINARY_DIR}/blas_config.txt "${data}")
endmacro()

macro(set_blas_definition_from_file filename)
    identify_blas(filename)
    target_compile_definitions(hasty_blas_c PUBLIC HASTY_BLAS_BLAS_${BLAS_LIB})
endmacro()

macro(set_blas_definition name)
    target_compile_definitions(hasty_blas_c PUBLIC HASTY_BLAS_HAS_BLAS)
    target_compile_definitions(hasty_blas_c PUBLIC HASTY_BLAS_IMPL_${name})
endmacro()

macro(download_openblas)
    message(STATUS "[ HASTY_BLAS_C ] Downloading OpenBLAS Build...")

    FetchContent_Declare(
            BuildOpenBLAS
            GIT_REPOSITORY https://github.com/LibRapid/BuildOpenBLAS.git
    )

    FetchContent_MakeAvailable(BuildOpenBLAS)

    set(BLAS_FOUND TRUE)

    if (${IS_WINDOWS})
        # Use openblas-windows-latest
        set(BLAS_LIBRARIES "${FETCHCONTENT_BASE_DIR}/buildopenblas-src/openblas-windows-latest/lib/openblas.lib")
    elseif (${IS_MACOS})
        # Use openblas-macos-latest
        set(BLAS_LIBRARIES "${FETCHCONTENT_BASE_DIR}/buildopenblas-src/openblas-macos-latest/lib/libopenblas.a")
    elseif (${IS_LINUX}) # Linux and other systems
        # Use openblas-ubuntu-latest
        set(BLAS_LIBRARIES "${FETCHCONTENT_BASE_DIR}/buildopenblas-src/openblas-ubuntu-latest/lib/libopenblas.so")
    else ()
        message(FATAL_ERROR "Pre-built OpenBLAS binaries are not available for this platform")
    endif ()

    set_blas_definition("OPENBLAS")
    set(HASTY_BLAS_C_BLAS ${BLAS_LIBRARIES})
endmacro()

macro(link_openblas)
    get_filename_component(filepath ${HASTY_BLAS_BLAS} DIRECTORY)
    get_filename_component(filename ${HASTY_BLAS_BLAS} NAME)

    set(include_path "${filepath}/../include")
    target_include_directories(hasty_blas_c PUBLIC "${include_path}")

    set(include_files "")
    if (EXISTS "${include_path}/openblas")
        FILE(GLOB_RECURSE include_files "${include_path}/openblas/*.*")
        target_include_directories(hasty_blas_c PUBLIC "${include_path}/openblas")
    else ()
        FILE(GLOB_RECURSE include_files "${include_path}/*.*")
    endif ()

    set(has_cblas OFF)

    foreach (file IN LISTS include_files)
        get_filename_component(inc_file ${file} NAME)
        if (${inc_file} STREQUAL "cblas.h")
            set(has_cblas ON)
        endif ()
    endforeach ()

    if (${has_cblas})
        write_blas_config(${HASTY_BLAS_C_BLAS})
        target_link_libraries(hasty_blas_c PUBLIC ${HASTY_BLAS_C_BLAS})
        set_blas_definition("OPENBLAS")
    else ()
        message(WARNING "[ HASTY_BLAS_C ] OpenBLAS does not contain cblas.h")
    endif ()
endmacro()

macro(link_accelerate)
    write_blas_config("accelerate")
    target_link_libraries(hasty_blas_c PUBLIC "-framework Accelerate")

    # If not using apple-clang, we need to relax some conditions
    if (NOT CMAKE_C_COMPILER_ID MATCHES "AppleClang")
        message(WARNING "[ HASTY_BLAS_C ] Accelerate is designed for AppleClang. Relaxing some conditions")
        target_compile_options(hasty_blas_c PUBLIC "-flax-vector-conversions")
    endif ()

    message(STATUS "Linking Apple Accelerate")
    set_blas_definition("ACCELERATE")
endmacro()

macro(link_generic)
endmacro()

macro(configure_blas)
    if (HASTY_BLAS_C_GET_BLAS)
        download_openblas()
    else ()
        find_package(BLAS REQUIRED)
    endif ()

    if (BLAS_FOUND)
        message(STATUS "[ HASTY_BLAS_C ] Located BLAS at ${BLAS_LIBRARIES}")

        list(GET ${BLAS_LIBRARIES} 0 HASTY_BLAS_BLAS)

        if (NOT ${HASTY_BLAS_BLAS})
            set(HASTY_BLAS_C_BLAS ${BLAS_LIBRARIES})
        endif ()

        message(STATUS "[ HASTY_BLAS_C ] Using BLAS")

        identify_blas("${HASTY_BLAS_C_BLAS}")

        # Configure BLAS (different steps are needed for each library)
        if (${BLAS_LIB} STREQUAL "OPENBLAS")
            link_openblas()
        elseif (${BLAS_LIB} STREQUAL "MKLBLAS")
            link_mkl()
        elseif (${BLAS_LIB} STREQUAL "ATLAS")
            link_atlas()
        elseif (${BLAS_LIB} STREQUAL "ACCELERATE")
            link_accelerate()
        else ()
            link_generic()
        endif ()
    else ()
        message(STATUS "[ HASTY_BLAS_C ] BLAS library not found on system. Consider enabling HASTY_BLAS_C_GET_BLAS")
    endif ()
endmacro()
