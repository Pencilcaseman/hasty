#include <hasty_blas_c.h>

void hasty_blas_sgemv(
  enum CBLAS_ORDER order,
  enum CBLAS_TRANSPOSE trans,
  uint64_t m,
  uint64_t n,
  float alpha,
  const float *a,
  uint64_t lda,
  const float *x,
  uint64_t inc_x,
  float beta,
  float *y,
  uint64_t inc_y
) {
  cblas_sgemv(
    order,
    trans,
    m,
    n,
    alpha,
    a,
    lda,
    x,
    inc_x,
    beta,
    y,
    inc_y
  );
}


void hasty_blas_dgemv(
  enum CBLAS_ORDER order,
  enum CBLAS_TRANSPOSE trans,
  uint64_t m,
  uint64_t n,
  double alpha,
  const double *a,
  uint64_t lda,
  const double *x,
  uint64_t inc_x,
  double beta,
  double *y,
  uint64_t inc_y
) {
  cblas_dgemv(
    order,
    trans,
    m,
    n,
    alpha,
    a,
    lda,
    x,
    inc_x,
    beta,
    y,
    inc_y
  );
}
