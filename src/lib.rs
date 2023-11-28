pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn test(x: i32) -> i32 {
    hasty_blas::test_function(x)
}

pub use hasty_blas::test_function;

pub enum StorageOrder {
    RowMajor,
    ColMajor,
}

pub enum Transpose {
    NoTrans,
    Conj,
    Trans,
    ConjTrans,
}

fn order_ffi(order: StorageOrder) -> hasty_blas::StorageOrder {
    match order {
        StorageOrder::RowMajor => hasty_blas::StorageOrder_RowMajor,
        StorageOrder::ColMajor => hasty_blas::StorageOrder_ColMajor,
    }
}

fn transpose_ffi(transpose: Transpose) -> hasty_blas::Transpose {
    match transpose {
        Transpose::NoTrans => hasty_blas::Transpose_NoTrans,
        Transpose::Conj => hasty_blas::Transpose_Conj,
        Transpose::Trans => hasty_blas::Transpose_Trans,
        Transpose::ConjTrans => hasty_blas::Transpose_ConjTrans,
    }
}

pub mod level3 {
    use crate::{order_ffi, StorageOrder, Transpose, transpose_ffi};

    trait Gemm where Self: Sized {
        fn gemm(order: StorageOrder,
                trans_a: Transpose,
                trans_b: Transpose,
                m: u64,
                n: u64,
                k: u64,
                alpha: Self,
                a: &Vec<Self>,
                lda: u64,
                b: &Vec<Self>,
                ldb: u64,
                beta: Self,
                c: &mut Vec<Self>,
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
                a: &Vec<Self>,
                lda: u64,
                b: &Vec<Self>,
                ldb: u64,
                beta: Self,
                c: &mut Vec<Self>,
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
                a: &Vec<Self>,
                lda: u64,
                b: &Vec<Self>,
                ldb: u64,
                beta: Self,
                c: &mut Vec<Self>,
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

    pub fn gemm<T: Gemm>(
        order: StorageOrder,
        trans_a: Transpose,
        trans_b: Transpose,
        m: u64,
        n: u64,
        k: u64,
        alpha: T,
        a: &Vec<T>,
        lda: u64,
        b: &Vec<T>,
        ldb: u64,
        beta: T,
        c: &mut Vec<T>,
        ldc: u64
    ) {
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
            ldc
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
