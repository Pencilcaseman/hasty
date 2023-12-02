#include <hasty_blas_c.h>

enum HastyBlasImpl hasty_blas_get_impl() {
    #if defined(HASTY_BLAS_IMPL_ACCELERATE)
    return HastyBlasImplAccelerate;
    #elif defined(HASTY_BLAS_IMPL_OPENBLAS)
    return HastyBlasImplOpenBlas;
    #elif defined(HASTY_BLAS_IMPL_MKL)
    return HastyBlasImplMkl;
    #endif

    return HastyBlasImplGeneric;
}
