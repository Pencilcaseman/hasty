#ifndef HASTY_BLAS_C_GEMV_H
#define HASTY_BLAS_C_GEMV_H
#if defined(HASTY_IMPL_HAS_BLAS)

#include "../hasty_blas.h"

void hasty_blas_sgemv(
    enum CBLAS_ORDER order,
    enum CBLAS_TRANSPOSE trans,
    uint64_t m,
    uint64_t n,
    float alpha,
    const float *a,
    uint64_t lda,
    const float *x,
    int64_t inc_x,
    float beta,
    float *y,
    int64_t inc_y
);


void hasty_blas_dgemv(
    enum CBLAS_ORDER order,
    enum CBLAS_TRANSPOSE trans,
    uint64_t m,
    uint64_t n,
    double alpha,
    const double *a,
    uint64_t lda,
    const double  *x,
    int64_t inc_x,
    double beta,
    double *y,
    int64_t inc_y
);

#endif // HASTY_IMPL_HAS_BLAS
#endif // HASTY_BLAS_C_GEMV_H
