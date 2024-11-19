use std::simd;

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

struct PtrWrapper<T>(*const T);
unsafe impl<T> Sync for PtrWrapper<T> {}

struct PtrWrapperMut<T>(*mut T);
unsafe impl<T> Sync for PtrWrapperMut<T> {}

pub fn mm_brute_safe(a: &[f32], b: &[f32], c: &mut [f32], m: usize, k: usize, n: usize) {
    for c_row in 0..m {
        for c_col in 0..n {
            for b_row in 0..k {
                c[c_row * n + c_col] += a[c_row * k + b_row] * b[b_row * n + c_col]
            }
        }
    }
}

pub fn mm_loop_reorder_safe(a: &[f32], b: &[f32], c: &mut [f32], m: usize, k: usize, n: usize) {
    for c_row in 0..m {
        for b_row in 0..k {
            for c_col in 0..n {
                c[c_row * n + c_col] += a[c_row * k + b_row] * b[b_row * n + c_col]
            }
        }
    }
}

pub fn mm_loop_reorder(a: *const f32, b: *const f32, c: *mut f32, m: usize, k: usize, n: usize) {
    for c_row in 0..m {
        let a_start = unsafe { a.add(c_row * k) };
        let c_start = unsafe { c.add(c_row * n) };
        for b_row in 0..k {
            let b_start = unsafe { b.add(b_row * n) };
            for c_col in 0..n {
                let a_val = *unsafe { a_start.add(b_row).as_ref().unwrap_unchecked() };
                let b_val = *unsafe { b_start.add(c_col).as_ref().unwrap_unchecked() };
                let c_ref = unsafe { c_start.add(c_col).as_mut().unwrap_unchecked() };

                *c_ref += a_val * b_val;
            }
        }
    }
}

pub fn mm_loop_reorder_rayon(
    a: *const f32,
    b: *const f32,
    c: *mut f32,
    m: usize,
    k: usize,
    n: usize,
) {
    let a = &PtrWrapper(a);
    let b = &PtrWrapper(b);
    let c = &PtrWrapperMut(c);

    (0..m).into_par_iter().for_each(|c_row| {
        let a = a.0;
        let b = b.0;
        let c = c.0;

        let a_start = unsafe { a.add(c_row * k) };
        let c_start = unsafe { c.add(c_row * n) };
        for b_row in 0..k {
            let b_start = unsafe { b.add(b_row * n) };
            let a_val = *unsafe { a_start.add(b_row).as_ref().unwrap_unchecked() };

            for c_col in 0..n {
                let b_val = *unsafe { b_start.add(c_col).as_ref().unwrap_unchecked() };
                let c_ref = unsafe { c_start.add(c_col).as_mut().unwrap_unchecked() };

                *c_ref += a_val * b_val;
            }
        }
    });
}

pub fn mm_tiling(a: *const f32, b: *const f32, c: *mut f32, m: usize, l: usize, n: usize) {
    // TODO: handle case when m,l,n % chunk_size != 0
    const TILE_SIZE: usize = 8;

    (0..m).step_by(TILE_SIZE).for_each(|i_high| {
        (0..l).step_by(TILE_SIZE).for_each(|k_high| {
            (0..n).step_by(TILE_SIZE).for_each(|j_high| {
                for i_low in 0..TILE_SIZE {
                    let i = i_high + i_low;
                    let a_start = unsafe { a.add(i * l) };
                    let c_start = unsafe { c.add(i * n) };

                    for k_low in 0..TILE_SIZE {
                        let k = k_high + k_low;
                        let b_start = unsafe { b.add(k * n) };
                        let a_val = *unsafe { a_start.add(k).as_ref().unwrap_unchecked() };

                        for j_low in 0..TILE_SIZE {
                            let j = j_high + j_low;

                            let b_val = *unsafe { b_start.add(j).as_ref().unwrap_unchecked() };
                            let c_ref = unsafe { c_start.add(j).as_mut().unwrap_unchecked() };

                            *c_ref += a_val * b_val;
                        }
                    }
                }
            });
        });
    });
}

pub fn mm_tiling_rayon(a: *const f32, b: *const f32, c: *mut f32, m: usize, l: usize, n: usize) {
    // TODO: handle case when m,l,n % chunk_size != 0
    const TILE_SIZE: usize = 8;

    let a = &PtrWrapper(a);
    let b = &PtrWrapper(b);
    let c = &PtrWrapperMut(c);

    (0..m)
        .into_par_iter()
        .step_by(TILE_SIZE)
        .for_each(|i_high| {
            (0..l).step_by(TILE_SIZE).for_each(|k_high| {
                (0..n).step_by(TILE_SIZE).for_each(|j_high| {
                    let a = a.0;
                    let b = b.0;
                    let c = c.0;

                    for i_low in 0..TILE_SIZE {
                        let i = i_high + i_low;
                        let a_start = unsafe { a.add(i * l) };
                        let c_start = unsafe { c.add(i * n) };

                        for k_low in 0..TILE_SIZE {
                            let k = k_high + k_low;
                            let b_start = unsafe { b.add(k * n) };
                            let a_val = *unsafe { a_start.add(k).as_ref().unwrap_unchecked() };

                            for j_low in 0..TILE_SIZE {
                                let j = j_high + j_low;

                                let b_val = *unsafe { b_start.add(j).as_ref().unwrap_unchecked() };
                                let c_ref = unsafe { c_start.add(j).as_mut().unwrap_unchecked() };

                                *c_ref += a_val * b_val;
                            }
                        }
                    }
                });
            });
        });
}

pub fn mm_tiling_rayon_vectorized(
    a: *const f32,
    b: *const f32,
    c: *mut f32,
    m: usize,
    l: usize,
    n: usize,
) {
    // TODO: handle case when m,l,n % chunk_size != 0
    const TILE_SIZE: usize = 8;
    type VecType = simd::f32x8;

    let a = &PtrWrapper(a);
    let b = &PtrWrapper(b);
    let c = &PtrWrapperMut(c);

    (0..m)
        .into_par_iter()
        .step_by(TILE_SIZE)
        .for_each(|i_high| {
            (0..l).step_by(TILE_SIZE).for_each(|k_high| {
                (0..n).step_by(TILE_SIZE).for_each(|j_high| {
                    let a = a.0;
                    let b = b.0;
                    let c = c.0;

                    for i_low in 0..TILE_SIZE {
                        let i = i_high + i_low;
                        let a_start = unsafe { a.add(i * l) };
                        let c_start = unsafe { c.add(i * n) };

                        for k_low in 0..TILE_SIZE {
                            let k = k_high + k_low;
                            let a_val = *unsafe { a_start.add(k).as_ref().unwrap_unchecked() };

                            let b_start = unsafe { b.add(k * n + j_high) };
                            let c_start = unsafe { c_start.add(j_high) };

                            let b_slice = unsafe { std::slice::from_raw_parts(b_start, TILE_SIZE) };
                            let c_slice =
                                unsafe { std::slice::from_raw_parts_mut(c_start, TILE_SIZE) };

                            let a_vec = VecType::from_array([a_val; TILE_SIZE]);
                            let b_vec = VecType::from_slice(&b_slice);
                            let mut c_vec = VecType::from_slice(&c_slice);
                            c_vec += a_vec * b_vec;

                            c_slice.copy_from_slice(c_vec.as_array());
                        }
                    }
                });
            });
        });
}
