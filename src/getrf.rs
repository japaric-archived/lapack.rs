//! LU factorization of a matrix

use complex::Complex;
use libc::c_int;

use ffi;

/// The signature of `getrf`
pub type Fn<T> = unsafe extern "C" fn (
    *const c_int,
    *const c_int,
    *mut T,
    *const c_int,
    *mut c_int,
    *mut c_int,
);

impl ::Getrf for Complex<f32> {
    fn getrf() -> Fn<Complex<f32>> {
        ffi::cgetrf_
    }
}

impl ::Getrf for Complex<f64> {
    fn getrf() -> Fn<Complex<f64>> {
        ffi::zgetrf_
    }
}

impl ::Getrf for f32 {
    fn getrf() -> Fn<f32> {
        ffi::sgetrf_
    }
}

impl ::Getrf for f64 {
    fn getrf() -> Fn<f64> {
        ffi::dgetrf_
    }
}
