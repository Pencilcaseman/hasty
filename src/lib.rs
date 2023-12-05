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
#![doc(
html_favicon_url = "https://raw.githubusercontent.com/Pencilcaseman/hasty/master/img/logo_dark_mode.png"
)]
#![doc(
html_logo_url = "https://raw.githubusercontent.com/Pencilcaseman/hasty/master/img/logo_dark_mode.png"
)]

pub mod errors;
mod hasty_blas_c;

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
fn order_ffi(order: StorageOrder) -> hasty_blas_c::CBLAS_ORDER {
    match order {
        StorageOrder::RowMajor => hasty_blas_c::CBLAS_ORDER_CblasRowMajor,
        StorageOrder::ColMajor => hasty_blas_c::CBLAS_ORDER_CblasColMajor,
    }
}

/// Given a transpose operation, convert it to the FFI representation.
fn transpose_ffi(transpose: Transpose) -> hasty_blas_c::CBLAS_TRANSPOSE {
    match transpose {
        Transpose::NoTrans => hasty_blas_c::CBLAS_TRANSPOSE_CblasNoTrans,
        Transpose::Conj => hasty_blas_c::CBLAS_TRANSPOSE_CblasConjNoTrans,
        Transpose::Trans => hasty_blas_c::CBLAS_TRANSPOSE_CblasTrans,
        Transpose::ConjTrans => hasty_blas_c::CBLAS_TRANSPOSE_CblasConjTrans,
    }
}

fn validate_ld<T>(
    order: &StorageOrder,
    trans: &Transpose,
    rows: &u64,
    cols: &u64,
    ld: &u64,
    err: T,
) -> Result<(), T> {
    // If using column major ordering, swap rows and cols
    let (rows, cols) = match order {
        StorageOrder::RowMajor => (rows, cols),
        StorageOrder::ColMajor => (cols, rows),
    };

    // If transposed, swap rows and cols again
    let cols = match trans {
        Transpose::NoTrans | Transpose::Conj => cols,
        Transpose::Trans | Transpose::ConjTrans => rows,
    };

    // Now, leading dimension must be at least max(1, cols)
    if ld >= std::cmp::max(&1, cols) {
        Ok(())
    } else {
        Err(err)
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
    let lib = unsafe { hasty_blas_c::hasty_blas_get_impl() };
    match lib {
        hasty_blas_c::HastyBlasImpl_HastyBlasImplGeneric => BlasLibrary::Generic,
        hasty_blas_c::HastyBlasImpl_HastyBlasImplAccelerate => BlasLibrary::Accelerate,
        hasty_blas_c::HastyBlasImpl_HastyBlasImplOpenBlas => BlasLibrary::OpenBlas,
        hasty_blas_c::HastyBlasImpl_HastyBlasImplMkl => BlasLibrary::Mkl,
        _ => panic!("Unknown BLAS library"),
    }
}

/// Level 3 BLAS routines, which perform matrix-matrix operations.
pub mod level3 {
    /// Trait for general matrix multiplication.
    pub trait Gemm
        where
            Self: Sized,
    {
        /// General matrix multiplication. See [`gemm`](fn.gemm.html) for more
        /// information.
        fn gemm(
            order: crate::StorageOrder,
            trans_a: crate::Transpose,
            trans_b: crate::Transpose,
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
            ldc: u64,
        );
    }

    impl Gemm for f32 {
        fn gemm(
            order: crate::StorageOrder,
            trans_a: crate::Transpose,
            trans_b: crate::Transpose,
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
            ldc: u64,
        ) {
            unsafe {
                crate::hasty_blas_c::hasty_blas_sgemm(
                    crate::order_ffi(order),
                    crate::transpose_ffi(trans_a),
                    crate::transpose_ffi(trans_b),
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
        fn gemm(
            order: crate::StorageOrder,
            trans_a: crate::Transpose,
            trans_b: crate::Transpose,
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
            ldc: u64,
        ) {
            unsafe {
                crate::hasty_blas_c::hasty_blas_dgemm(
                    crate::order_ffi(order),
                    crate::transpose_ffi(trans_a),
                    crate::transpose_ffi(trans_b),
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
    /// Compute `c := alpha * op(a) * op(b) + beta * c`, where `a` is an
    /// `m x k` matrix, `b` is a `k x n` matrix, `c` is an `m x n` matrix,
    /// `alpha` and `beta` are scalars, and `op` is one of:
    /// * `op(x) = x`
    /// * `op(x) = x^T`
    /// * `op(x) = x^H`
    /// where `x` is a matrix.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the matrix elements
    ///
    /// # Parameters
    ///
    /// * `order`: The storage order of the matrices
    /// * `trans_a`: Whether to transpose `a`
    /// * `trans_b`: Whether to transpose `b`
    /// * `m`: The number of rows in `a` and `c`
    /// * `n`: The number of columns in `b` and `c`
    /// * `k`: The number of columns in `a` and rows in `b`
    /// * `alpha`: The scalar `alpha`
    /// * `a`: The matrix `a`
    /// * `lda`: The leading dimension of `a`
    /// * `b`: The matrix `b`
    /// * `ldb`: The leading dimension of `b`
    /// * `beta`: The scalar `beta`
    /// * `c`: The matrix `c`
    /// * `ldc`: The leading dimension of `c`
    ///
    /// # Panics
    ///
    /// * `a.len() != m * k`
    /// * `b.len() != k * n`
    /// * `c.len() != m * n`
    /// * `lda < cols of op(a)`
    /// * `ldb < cols of op(b)`
    /// * `ldc < n`
    ///
    /// # Example
    ///
    /// ```none
    /// [[1 2 3]    [[1]      [[14]
    ///  [4 5 6]] x  [2]   =   [32]]
    ///              [3]]
    /// ```
    ///
    /// ```rust
    /// let m: u64 = 2;
    /// let n: u64 = 1;
    /// let k: u64 = 3;
    ///
    /// let a: Vec<f32> = vec![1.0, 2.0, 3.0,
    ///                        4.0, 5.0, 6.0];
    /// let b: Vec<f32> = vec![1.0,
    ///                        2.0,
    ///                        3.0];
    /// let mut c: Vec<f32> = vec![0.0,
    ///                            0.0];
    ///
    /// let alpha: f32 = 1.0;
    /// let beta: f32 = 0.0;
    ///
    /// // Calculate the product of a and b, storing the result in c
    /// if let Err(e) = hasty::level3::gemm(
    ///     hasty::StorageOrder::RowMajor,
    ///     hasty::Transpose::NoTrans,
    ///     hasty::Transpose::NoTrans,
    ///     m,
    ///     n,
    ///     k,
    ///     1.0,
    ///     &a,
    ///     k,
    ///     &b,
    ///     n,
    ///     0.0,
    ///     &mut c,
    ///     n,
    /// ) {
    ///     // Handle the various errors that could occur. Most of the time, you can
    ///     // probably just panic or print the error
    ///     use hasty::errors::GemmError;
    ///     match e {
    ///         GemmError::MatA => println!("Invalid Matrix A"),
    ///         GemmError::MatB => println!("Invalid Matrix B"),
    ///         GemmError::MatC => println!("Invalid Matrix C"),
    ///         GemmError::Lda => println!("Invalid Lda"),
    ///         GemmError::Ldb => println!("Invalid Ldb"),
    ///         GemmError::Ldc => println!("Invalid Ldc"),
    ///     }
    /// }
    ///
    /// println!("Result: {:?}", c); // [14.0, 32.0]
    /// ```
    pub fn gemm<T: Gemm>(
        order: crate::StorageOrder,
        trans_a: crate::Transpose,
        trans_b: crate::Transpose,
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
    ) -> Result<(), crate::errors::GemmError> {
        // Check dimensions and strides are valid
        if a.len() as u64 != m * k { return Err(crate::errors::GemmError::MatA); }
        if b.len() as u64 != k * n { return Err(crate::errors::GemmError::MatB); }
        if c.len() as u64 != m * n { return Err(crate::errors::GemmError::MatC); }

        crate::validate_ld(
            &order,
            &trans_a,
            &m,
            &k,
            &lda,
            crate::errors::GemmError::Lda,
        )?;

        crate::validate_ld(
            &order,
            &trans_b,
            &k,
            &n,
            &ldb,
            crate::errors::GemmError::Ldb,
        )?;

        crate::validate_ld(
            &order,
            &crate::Transpose::NoTrans,
            &m,
            &n,
            &ldc,
            crate::errors::GemmError::Ldc,
        )?;

        T::gemm(
            order, trans_a, trans_b, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
        );

        Ok(())
    }
}

/// Level 2 BLAS routines, which perform matrix-vector operations.
pub mod level2 {
    /// Trait for general matrix-vector multiplication
    pub trait Gemv
        where
            Self: Sized,
    {
        /// General matrix-vector multiplication. See [`gemv`](fn.gemv.html) for
        /// more information.
        fn gemv(
            order: crate::StorageOrder,
            trans: crate::Transpose,
            m: u64,
            n: u64,
            alpha: Self,
            a: &[Self],
            lda: u64,
            x: &[Self],
            inc_x: i64,
            beta: Self,
            y: &mut [Self],
            inc_y: i64,
        );
    }

    impl Gemv for f32 {
        fn gemv(
            order: crate::StorageOrder,
            trans: crate::Transpose,
            m: u64,
            n: u64,
            alpha: Self,
            a: &[Self],
            lda: u64,
            x: &[Self],
            inc_x: i64,
            beta: Self,
            y: &mut [Self],
            inc_y: i64,
        ) {
            unsafe {
                crate::hasty_blas_c::hasty_blas_sgemv(
                    crate::order_ffi(order),
                    crate::transpose_ffi(trans),
                    m,
                    n,
                    alpha,
                    a.as_ptr(),
                    lda,
                    x.as_ptr(),
                    inc_x,
                    beta,
                    y.as_mut_ptr(),
                    inc_y,
                );
            }
        }
    }

    /// General matrix-vector multiplication
    ///
    /// Compute `c := alpha * op(a) * x + beta * c`, where `a` is a
    /// `m x n` matrix, `x` is a vector with `n` elements, `c` is a
    /// vector with `m` elements, and `alpha` and `beta` are scalars.
    /// `op` is one of:
    /// * `op(x) = x`
    /// * `op(x) = x^T`
    /// * `op(x) = x^H`
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the matrix and vector elements
    ///
    /// # Parameters
    ///
    /// * `order`: The storage order of the matrix
    /// * `trans`: Whether to transpose `a`
    /// * `m`: The number of rows in `a` and elements in `y`
    /// * `n`: The number of columns in `a` and elements in `x`
    /// * `alpha`: The scalar `alpha`
    /// * `a`: The matrix `a`
    /// * `lda`: The leading dimension of `a`
    /// * `x`: The vector `x`
    /// * `inc_x`: The increment of `x`
    /// * `beta`: The scalar `beta`
    /// * `y`: The vector `y`
    /// * `inc_y`: The increment of `y`
    ///
    /// # Panics
    ///
    /// * `a.len() != m * n`
    /// * `x.len() != n`
    /// * `y.len() != m`
    /// * `lda < cols of op(a)`
    /// * `inc_x == 0`
    /// * `inc_y == 0`
    /// * `x.len() < 1 + (n - 1) * inc_x.unsigned_abs()`
    /// * `y.len() < 1 + (m - 1) * inc_y.unsigned_abs()`
    ///
    /// # Example
    ///
    /// ```none
    /// [[1 2 3]  x [1 2 3] = [14 32]
    ///  [4 5 6]]
    /// ```
    ///
    /// ```rust
    /// let a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// let b: Vec<f32> = vec![1.0, 2.0, 3.0];
    /// let mut c: Vec<f32> = vec![0.0, 0.0];
    ///
    /// // Compute matrix-vector product
    /// if let Err(e) = hasty::level2::gemv(
    ///     hasty::StorageOrder::RowMajor,
    ///     hasty::Transpose::NoTrans,
    ///     2,
    ///     3,
    ///     1.0,
    ///     &a,
    ///     3,
    ///     &b,
    ///     1,
    ///     0.0,
    ///     &mut c,
    ///     1,
    /// ) {
    ///     // Handle various errors -- you can probably just panic on most of these.
    ///     match e {
    ///         hasty::errors::GemvError::MatA => println!("Invalid Matrix A"),
    ///         hasty::errors::GemvError::VecX => println!("Invalid Vector X"),
    ///         hasty::errors::GemvError::VecY => println!("Invalid Vector Y"),
    ///         hasty::errors::GemvError::Lda => println!("Invalid Lda"),
    ///         hasty::errors::GemvError::IncX => println!("Invalid IncX"),
    ///         hasty::errors::GemvError::IncY => println!("Invalid IncY"),
    ///     }
    /// }
    ///
    /// println!("GEMV Result: {:?}", c);
    /// ```
    pub fn gemv<T: Gemv>(
        order: crate::StorageOrder,
        trans: crate::Transpose,
        m: u64,
        n: u64,
        alpha: T,
        a: &[T],
        lda: u64,
        x: &[T],
        inc_x: i64,
        beta: T,
        y: &mut [T],
        inc_y: i64,
    ) -> Result<(), crate::errors::GemvError> {
        if a.len() != (m * n) as usize { return Err(crate::errors::GemvError::MatA); }
        if x.len() != n as usize { return Err(crate::errors::GemvError::VecX); }
        if y.len() != m as usize { return Err(crate::errors::GemvError::VecY); }

        crate::validate_ld(
            &order,
            &trans,
            &m,
            &n,
            &lda,
            crate::errors::GemvError::Lda,
        )?;

        if inc_x == 0 { return Err(crate::errors::GemvError::IncX); }
        if inc_y == 0 { return Err(crate::errors::GemvError::IncY); }

        if x.len() < (1 + (n - 1) * inc_x.unsigned_abs()) as usize { return Err(crate::errors::GemvError::IncX); }
        if y.len() < (1 + (m - 1) * inc_y.unsigned_abs()) as usize { return Err(crate::errors::GemvError::IncY); }

        T::gemv(order, trans, m, n, alpha, a, lda, x, inc_x, beta, y, inc_y);

        Ok(())
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
