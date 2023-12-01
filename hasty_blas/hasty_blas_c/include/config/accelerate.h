#ifndef HASTY_BLAS_C_ACCELERATE_H
#define HASTY_BLAS_C_ACCELERATE_H

#ifdef BLASINT
#	define CBLAS_INT BLASINT
#else
#	define CBLAS_INT int
#endif

#define BLAS_IMPL "Accelerate.framework"

#ifndef CBLAS_INDEX
#	define CBLAS_INDEX size_t
#endif // CBLAS_INDEX

#define ACCELERATE_NEW_LAPACK 1
#include <Accelerate/Accelerate.h>

#ifdef negativeInfinity // This breaks things
#undef negativeInfinity
#endif

#endif // HASTY_BLAS_C_ACCELERATE_H