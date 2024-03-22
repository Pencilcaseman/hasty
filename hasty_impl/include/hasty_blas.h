#ifndef HASTY_BLAS_H
#define HASTY_BLAS_H
#ifdef HASTY_IMPL_HAS_BLAS

#include <stdint.h>

#ifndef HASTY_IMPL_BLAS_ACCELERATE
enum CBLAS_ORDER { CblasRowMajor = 101, CblasColMajor = 102 };

enum CBLAS_TRANSPOSE {
	CblasNoTrans	 = 111,
	CblasTrans		 = 112,
	CblasConjTrans	 = 113,
	CblasConjNoTrans = 114
};

enum CBLAS_UPLO { CblasUpper = 121, CblasLower = 122 };
enum CBLAS_DIAG { CblasNonUnit = 131, CblasUnit = 132 };
enum CBLAS_SIDE { CblasLeft = 141, CblasRight = 142 };
#endif

enum HastyBlasImpl {
    HastyBlasImplGeneric,
    HastyBlasImplAccelerate,
    HastyBlasImplOpenBlas,
    HastyBlasImplMkl,
};

enum HastyBlasImpl hasty_blas_get_impl();

#if defined(HASTY_IMPL_BLAS_ACCELERATE)
#   include <config/accelerate.h>
#elif defined(HASTY_IMPL_BLAS_OPENBLAS)
#   include <config/openblas.h>
#endif

#include "level2/gemv.h"
#include "level3/gemm.h"

#endif // HASTY_IMPL_HAS_BLAS
#endif // HASTY_BLAS_H
