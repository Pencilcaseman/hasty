#ifndef HASTY_BLAS_C_HPP
#define HASTY_BLAS_C_HPP

#include <stdint.h>

enum StorageOrder {
    RowMajor = 101,
    ColMajor = 102
};

enum Transpose {
    NoTrans = 111,
    Conj = 114,
    Trans = 112,
    ConjTrans = 113,
};

int hasty_test_function(int x);

#ifdef HASTY_BLAS_IMPL_ACCELERATE
#   include <config/accelerate.h>
#endif

#include "level3/gemm.h"

#endif // HASTY_BLAS_C_HPP