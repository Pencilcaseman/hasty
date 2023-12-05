//! # Hasty Errors
//!
//! Contains a range of error enums which are returned by
//! BLAS functions to indicate what, if anything, went
//! wrong with the function call

use std::fmt::Formatter;

/// An enum representing the different possible errors
/// the [`gemm`](fn.gemm.html) function can output
#[derive(PartialEq)]
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

impl std::fmt::Debug for GemmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GemmError::MatA => write!(f, "Invalid Matrix A"),
            GemmError::MatB => write!(f, "Invalid Matrix B"),
            GemmError::MatC => write!(f, "Invalid Matrix C"),
            GemmError::Lda => write!(f, "Invalid Lda"),
            GemmError::Ldb => write!(f, "Invalid Ldb"),
            GemmError::Ldc => write!(f, "Invalid Ldc"),
        }
    }
}

impl std::fmt::Display for GemmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GemmError::MatA => write!(f, "Invalid Matrix A"),
            GemmError::MatB => write!(f, "Invalid Matrix B"),
            GemmError::MatC => write!(f, "Invalid Matrix C"),
            GemmError::Lda => write!(f, "Invalid Lda"),
            GemmError::Ldb => write!(f, "Invalid Ldb"),
            GemmError::Ldc => write!(f, "Invalid Ldc"),
        }
    }
}

/// An enum representing the different possible errors
/// the [`gemv`](fn.gemv.html) function can output
#[derive(PartialEq)]
pub enum GemvError {
    /// Matrix A has invalid size (must be `m x n`)
    MatA,
    /// Vector X has invalid size (must be `n`)
    VecX,
    /// Vector Y has invalid size (must be `m`)
    VecY,
    /// LDA is invalid (must be at least `n`)
    Lda,
    /// IncX is invalid (cannot be zero)
    IncX,
    /// IncY is invalid (cannot be zero)
    IncY,
}

impl std::fmt::Debug for GemvError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GemvError::MatA => write!(f, "Invalid Matrix A"),
            GemvError::VecX => write!(f, "Invalid Vector X"),
            GemvError::VecY => write!(f, "Invalid Vector Y"),
            GemvError::Lda => write!(f, "Invalid Lda"),
            GemvError::IncX => write!(f, "Invalid IncX"),
            GemvError::IncY => write!(f, "Invalid IncY"),
        }
    }
}

impl std::fmt::Display for GemvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GemvError::MatA => write!(f, "Invalid Matrix A"),
            GemvError::VecX => write!(f, "Invalid Vector X"),
            GemvError::VecY => write!(f, "Invalid Vector Y"),
            GemvError::Lda => write!(f, "Invalid Lda"),
            GemvError::IncX => write!(f, "Invalid IncX"),
            GemvError::IncY => write!(f, "Invalid IncY"),
        }
    }
}
