#ifndef HASTY_BLAS_C_GEMM_H
#define HASTY_BLAS_C_GEMM_H

void hasty_blas_sgemm(
            enum StorageOrder order,
            enum Transpose trans_a,
            enum Transpose trans_b,
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
);

void hasty_blas_dgemm(
            enum StorageOrder order,
            enum Transpose trans_a,
            enum Transpose trans_b,
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
);

#endif // HASTY_BLAS_C_GEMM_H
