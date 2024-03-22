#if defined(HASTY_IMPL_HAS_BLAS)

#include <hasty_blas.h>

void hasty_blas_sgemm(
        enum CBLAS_ORDER order,
        enum CBLAS_TRANSPOSE trans_a,
        enum CBLAS_TRANSPOSE trans_b,
        uint64_t m,
        uint64_t n,
        uint64_t k,
        float alpha,
        const float *a,
        uint64_t lda,
        const float *b,
        uint64_t ldb,
        float beta,
        float *c,
        uint64_t ldc
) {
    cblas_sgemm(
        order,
        trans_a,
        trans_b,
        m,
        n,
        k,
        alpha,
        a,
        lda,
        b,
        ldb,
        beta,
        c,
        ldc
    );
}

void hasty_blas_dgemm(
        enum CBLAS_ORDER order,
        enum CBLAS_TRANSPOSE trans_a,
        enum CBLAS_TRANSPOSE trans_b,
        uint64_t m,
        uint64_t n,
        uint64_t k,
        double alpha,
        const double *a,
        uint64_t lda,
        const double *b,
        uint64_t ldb,
        double beta,
        double *c,
        uint64_t ldc
) {
    cblas_dgemm(
        order,
        trans_a,
        trans_b,
        m,
        n,
        k,
        alpha,
        a,
        lda,
        b,
        ldb,
        beta,
        c,
        ldc
    );
}

#else

// Define a function so it doesn't complain
void gemm_placeholder() {}

#endif // HASTY_IMPL_HAS_BLAS
