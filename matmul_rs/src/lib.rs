#![feature(portable_simd)]

mod algos;
pub use algos::*;

pub struct Mat {
    pub data: Vec<f32>,
    pub height: usize,
    pub width: usize,
}

pub enum MatMulImpl {
    Safe(
        fn(
            /*A*/ &[f32],
            /*B*/ &[f32],
            /*C*/ &mut [f32],
            /*m*/ usize,
            /*n*/ usize,
            /*k*/ usize,
        ),
    ),
    Unsafe(
        fn(
            /*A*/ *const f32,
            /*B*/ *const f32,
            /*C*/ *mut f32,
            /*m*/ usize,
            /*n*/ usize,
            /*k*/ usize,
        ),
    ),
}

pub fn matmul(alg: &MatMulImpl, a: &Mat, b: &Mat, dst: &mut Mat) {
    match alg {
        MatMulImpl::Safe(f) => f(&a.data, &b.data, &mut dst.data, a.height, a.width, b.width),
        MatMulImpl::Unsafe(f) => {
            let m = a.height;
            let k = a.width;
            let n = b.width;

            assert!(a.data.len() <= m * k);
            assert!(b.data.len() <= k * n);
            assert!(dst.data.len() <= m * n);

            let a = a.data.as_ptr();
            let b = b.data.as_ptr();
            let dst = dst.data.as_mut_ptr();

            f(a, b, dst, m, k, n)
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    const A_DATA: &'static [u8; 3072] = include_bytes!("testdata/a.txt");
    const B_DATA: &'static [u8; 6144] = include_bytes!("testdata/b.txt");
    const C_DATA: &'static [u8; 4608] = include_bytes!("testdata/c.txt");

    fn bytes_to_f32(bytes: &[u8]) -> Vec<f32> {
        let mut aligned = vec![0.0f32; bytes.len() / 4];
        let aligned_slice = bytemuck::cast_slice_mut(&mut aligned);
        aligned_slice.copy_from_slice(&bytes);

        aligned
    }

    #[rstest::fixture]
    fn a() -> Mat {
        let data = bytes_to_f32(A_DATA);
        let height = 24;
        let width = 32;

        Mat {
            data,
            height,
            width,
        }
    }

    #[rstest::fixture]
    fn b() -> Mat {
        let data = bytes_to_f32(B_DATA);
        let height = 32;
        let width = 48;

        Mat {
            data,
            height,
            width,
        }
    }

    #[rstest::fixture]
    fn c() -> Mat {
        let height = 24;
        let width = 48;
        let data = vec![0.0; width * height];

        Mat {
            data,
            height,
            width,
        }
    }

    #[rstest::fixture]
    fn expected() -> Mat {
        let data = bytes_to_f32(C_DATA);
        let height = 32;
        let width = 48;

        Mat {
            data,
            height,
            width,
        }
    }

    #[rstest::rstest]
    fn algorithm_correct(
        #[values(
            MatMulImpl::Safe(mm_brute_safe),
            MatMulImpl::Safe(mm_loop_reorder_safe),
            MatMulImpl::Unsafe(mm_loop_reorder),
            MatMulImpl::Unsafe(mm_loop_reorder_rayon),
            MatMulImpl::Unsafe(mm_tiling),
            MatMulImpl::Unsafe(mm_tiling_rayon),
            MatMulImpl::Unsafe(mm_tiling_rayon_vectorized),
        )]
        f: MatMulImpl,
        a: Mat,
        b: Mat,
        mut c: Mat,
        expected: Mat,
    ) {
        matmul(&f, &a, &b, &mut c);

        let actual: &[f32] = &c.data;
        let expected: &[f32] = &expected.data;
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-5);
    }
}
