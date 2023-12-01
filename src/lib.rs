//! # Hasty
//!
//! Hasty is a Rust wrapper for BLAS libraries, making use of highly
//! optimised BLAS implementations found on the system. It finds these
//! libraries via CMake, which is run as part of the build process.
//!
//! To specify a particular BLAS library, set the `HASTY_BLAS_PATH`
//! environment variable to the absolute path to the library. If this
//! variable is not set, CMake will search for a library on the system.

#![warn(missing_docs)]
#![warn(clippy::pedantic, clippy::nursery)]

/// Represents the storage order of a matrix.
pub enum StorageOrder {
    /// Row-major storage order.
    RowMajor,

    /// Column-major storage order.
    ColMajor,
}

/// Represents the transpose operation.
pub enum Transpose {
    /// No transpose.
    NoTrans,

    /// Conjugate transpose
    Conj,

    /// Transpose
    Trans,

    /// Conjugate transpose
    ConjTrans,
}

/// Given a storage order, convert it to the FFI representation.
fn order_ffi(order: StorageOrder) -> hasty_blas::CBLAS_ORDER {
    match order {
        StorageOrder::RowMajor => hasty_blas::CBLAS_ORDER_CblasRowMajor,
        StorageOrder::ColMajor => hasty_blas::CBLAS_ORDER_CblasColMajor,
    }
}

/// Given a transpose operation, convert it to the FFI representation.
fn transpose_ffi(transpose: Transpose) -> hasty_blas::CBLAS_TRANSPOSE {
    match transpose {
        Transpose::NoTrans => hasty_blas::CBLAS_TRANSPOSE_CblasNoTrans,
        Transpose::Conj => hasty_blas::CBLAS_TRANSPOSE_CblasConjNoTrans,
        Transpose::Trans => hasty_blas::CBLAS_TRANSPOSE_CblasTrans,
        Transpose::ConjTrans => hasty_blas::CBLAS_TRANSPOSE_CblasConjTrans,
    }
}

/// An enum representing the BLAS library used in the backend.
#[derive(Debug)]
pub enum BlasLibrary {
    /// Generic BLAS implementation (probably not very fast).
    Generic,

    /// Apple's Accelerate framework.
    Accelerate,

    /// OpenBLAS.
    OpenBlas,

    /// Intel's MKL.
    Mkl,
}

impl std::fmt::Display for BlasLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlasLibrary::Generic => write!(f, "Generic"),
            BlasLibrary::Accelerate => write!(f, "Accelerate"),
            BlasLibrary::OpenBlas => write!(f, "OpenBLAS"),
            BlasLibrary::Mkl => write!(f, "MKL"),
        }
    }
}

/// Get the BLAS library begin used.
pub fn get_blas_library() -> BlasLibrary {
    let lib = unsafe { hasty_blas::hasty_blas_get_impl() };
    match lib {
        hasty_blas::HastyBlasImpl_HastyBlasImplGeneric => BlasLibrary::Generic,
        hasty_blas::HastyBlasImpl_HastyBlasImplAccelerate => BlasLibrary::Accelerate,
        hasty_blas::HastyBlasImpl_HastyBlasImplOpenBlas => BlasLibrary::OpenBlas,
        hasty_blas::HastyBlasImpl_HastyBlasImplMkl => BlasLibrary::Mkl,
        _ => panic!("Unknown BLAS library"),
    }
}

/// Level 3 BLAS routines, which perform matrix-matrix operations.
pub mod level3 {
    use crate::{order_ffi, StorageOrder, Transpose, transpose_ffi};

    /// Trait for general matrix multiplication.
    pub trait Gemm where Self: Sized {
        /// General matrix multiplication. See [`gemm`](fn.gemm.html) for more
        /// information.
        fn gemm(order: StorageOrder,
                trans_a: Transpose,
                trans_b: Transpose,
                m: u64,
                n: u64,
                k: u64,
                alpha: Self,
                a: &[Self],
                lda: u64,
                b: &[Self],
                ldb: u64,
                beta: Self,
                c: &mut [Self],
                ldc: u64);
    }

    impl Gemm for f32 {
        fn gemm(order: StorageOrder,
                trans_a: Transpose,
                trans_b: Transpose,
                m: u64,
                n: u64,
                k: u64,
                alpha: Self,
                a: &[Self],
                lda: u64,
                b: &[Self],
                ldb: u64,
                beta: Self,
                c: &mut [Self],
                ldc: u64) {
            unsafe {
                hasty_blas::hasty_blas_sgemm(
                    order_ffi(order),
                    transpose_ffi(trans_a),
                    transpose_ffi(trans_b),
                    m,
                    n,
                    k,
                    alpha,
                    a.as_ptr(),
                    lda,
                    b.as_ptr(),
                    ldb,
                    beta,
                    c.as_mut_ptr(),
                    ldc,
                );
            }
        }
    }

    impl Gemm for f64 {
        fn gemm(order: StorageOrder,
                trans_a: Transpose,
                trans_b: Transpose,
                m: u64,
                n: u64,
                k: u64,
                alpha: Self,
                a: &[Self],
                lda: u64,
                b: &[Self],
                ldb: u64,
                beta: Self,
                c: &mut [Self],
                ldc: u64) {
            unsafe {
                hasty_blas::hasty_blas_dgemm(
                    order_ffi(order),
                    transpose_ffi(trans_a),
                    transpose_ffi(trans_b),
                    m,
                    n,
                    k,
                    alpha,
                    a.as_ptr(),
                    lda,
                    b.as_ptr(),
                    ldb,
                    beta,
                    c.as_mut_ptr(),
                    ldc,
                );
            }
        }
    }

    /// General matrix multiplication.
    ///
    /// Compute $c := \alpha \text{op}_1(a) \text{op}_2(b) + \beta c$, where $a$ is a
    /// $m \times k$ matrix, $b$ is a $k \times n$ matrix, $c$ is a
    /// $m \times n$ matrix, and $\alpha$ and $\beta$ are scalars.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the matrix elements
    ///
    /// # Parameters
    ///
    /// * `order`: The storage order of the matrices
    /// * `trans_a`: Whether to transpose $a$
    /// * `trans_b`: Whether to transpose $b$
    /// * `m`: The number of rows in $a$ and $c$
    /// * `n`: The number of columns in $b$ and $c$
    /// * `k`: The number of columns in $a$ and rows in $b$
    /// * `alpha`: The scalar $\alpha$
    /// * `a`: The matrix $a$
    /// * `lda`: The leading dimension of $a$
    /// * `b`: The matrix $b$
    /// * `ldb`: The leading dimension of $b$
    /// * `beta`: The scalar $\beta$
    /// * `c`: The matrix $c$
    /// * `ldc`: The leading dimension of $c$
    ///
    /// # Panics
    ///
    /// * `a.len() != m * k`
    /// * `b.len() != k * n`
    /// * `c.len() != m * n`
    ///
    /// # Example
    ///
    /// $$\left( \begin{array}{cc}
    ///     1 & 2 & 3 \\\\
    ///     4 & 5 & 6
    /// \end{array} \right) \times
    /// \left( \begin{array}{cc}
    ///     1 \\\\
    ///     2 \\\\
    ///     3
    /// \end{array} \right) =
    /// \left( \begin{array}{cc}
    ///     14 \\\\
    ///     32
    /// \end{array} \right)$$
    ///
    /// ```rust
    /// let m: u64 = 2;
    /// let n: u64 = 1;
    /// let k: u64 = 3;
    /// let a: Vec<f32> = vec![1.0, 2.0, 3.0,
    ///                        4.0, 5.0, 6.0];
    /// let b: Vec<f32> = vec![1.0,
    ///                        2.0,
    ///                        3.0];
    /// let c: Vec<f32> = vec![0.0,
    ///                        0.0];
    /// let alpha: f32 = 1.0;
    /// let beta: f32 = 0.0;
    /// hasty::level3::gemm(
    ///     hasty::StorageOrder::RowMajor,
    ///     hasty::Transpose::NoTrans,
    ///     hasty::Transpose::NoTrans,
    ///     m,
    ///     n,
    ///     k,
    ///     alpha,
    ///     &a,
    ///     k,
    ///     &b,
    ///     n,
    ///     beta,
    ///     &mut c,
    ///     n);
    ///
    /// println!("Result: {:?}", c); // [14.0, 32.0]
    /// ```
    ///
    pub fn gemm<T: Gemm>(
        order: StorageOrder,
        trans_a: Transpose,
        trans_b: Transpose,
        m: u64,
        n: u64,
        k: u64,
        alpha: T,
        a: &[T],
        lda: u64,
        b: &[T],
        ldb: u64,
        beta: T,
        c: &mut [T],
        ldc: u64,
    ) {
        // Check dimensions and strides are valid
        assert_eq!(a.len() as u64, m * k);
        assert_eq!(b.len() as u64, k * n);
        assert_eq!(c.len() as u64, m * n);

        T::gemm(
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
            ldc,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemm() {
        let m: u64 = 2;
        let n: u64 = 1;
        let k: u64 = 3;
        let mut a: Vec<f32> = vec![0.0; (m * k) as usize];
        let mut b: Vec<f32> = vec![0.0; (k * n) as usize];
        let mut c: Vec<f32> = vec![0.0; (m * n) as usize];

        for i in 0..(m * k) {
            a[i as usize] = i as f32 + 1.0;
        }

        for i in 0..(k * n) {
            b[i as usize] = i as f32 + 1.0;
        }

        level3::gemm(
            StorageOrder::RowMajor,
            Transpose::NoTrans,
            Transpose::NoTrans,
            m,
            n,
            k,
            1.0,
            &a,
            k,
            &b,
            n,
            0.0,
            &mut c,
            n,
        );

        assert_eq!(c, vec![14.0, 32.0]);
    }
}
