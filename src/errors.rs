//! # Hasty Errors
//!
//! Contains a range of error enums which are returned by
//! BLAS functions to indicate what, if anything, went
//! wrong with the function call

/// An enum representing the different possible errors
/// the [`gemm`](fn.gemm.html) function can output
pub enum GemmError {
    /// Matrix A has invalid size (must be `m x k`)
    MatA,
    /// Matrix B has invalid size (must be `k x n`)
    MatB,
    /// Matrix C has invalid size (must be `m x n`)
    MatC,
    /// LDA is invalid (must be at least the number of
    /// columns of `op(A)`)
    Lda,
    /// LDB is invalid (must be at least the number of
    /// columns of `op(B)`)
    Ldb,
    /// LDC is invalid (must be at least `n`)
    Ldc,
}
