#if defined(HASTY_IMPL_HAS_BLAS)

#include <hasty_blas.h>

enum HastyBlasImpl hasty_blas_get_impl() {
    #if defined(HASTY_IMPL_BLAS_ACCELERATE)
    return HastyBlasImplAccelerate;
    #elif defined(HASTY_IMPL_BLAS_OPENBLAS)
    return HastyBlasImplOpenBlas;
    #elif defined(HASTY_IMPL_BLAS_MKL)
    return HastyBlasImplMkl;
    #endif

    return HastyBlasImplGeneric;
}

#else

void blas_placeholder() {}

#endif // HASTY_IMPL_HAS_BLAS
