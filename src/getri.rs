//! Invert a matrix using the LU factorization computed by GETRF

use complex::Complex;
use libc::c_int;

use ffi;

/// The signature of `getri`
pub type Fn<T> = unsafe extern "C" fn (
    *const c_int,
    *mut T,
    *const c_int,
    *const c_int,
    *mut T,
    *const c_int,
    *mut c_int,
);

impl ::Getri for Complex<f32> {
    fn getri() -> Fn<Complex<f32>> {
        ffi::cgetri_
    }
}

impl ::Getri for Complex<f64> {
    fn getri() -> Fn<Complex<f64>> {
        ffi::zgetri_
    }
}

impl ::Getri for f32 {
    fn getri() -> Fn<f32> {
        ffi::sgetri_
    }
}

impl ::Getri for f64 {
    fn getri() -> Fn<f64> {
        ffi::dgetri_
    }
}
