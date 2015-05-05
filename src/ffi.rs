use complex::Complex;

use libc::{c_double, c_float, c_int};

#[link(name = "lapack")]
extern "C" {
    pub fn sgetrf_(
        m: *const c_int,
        n: *const c_int,
        a: *mut c_float,
        lda: *const c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    );

    pub fn dgetrf_(
        m: *const c_int,
        n: *const c_int,
        a: *mut c_double,
        lda: *const c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    );

    pub fn cgetrf_(
        m: *const c_int,
        n: *const c_int,
        a: *mut Complex<c_float>,
        lda: *const c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    );

    pub fn zgetrf_(
        m: *const c_int,
        n: *const c_int,
        a: *mut Complex<c_double>,
        lda: *const c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    );
}

#[link(name = "lapack")]
extern "C" {
    pub fn sgetri_(
        n: *const c_int,
        a: *mut c_float,
        lda: *const c_int,
        ipiv: *const c_int,
        work: *mut c_float,
        lwork: *const c_int,
        info: *mut c_int,
    );

    pub fn dgetri_(
        n: *const c_int,
        a: *mut c_double,
        lda: *const c_int,
        ipiv: *const c_int,
        work: *mut c_double,
        lwork: *const c_int,
        info: *mut c_int,
    );

    pub fn cgetri_(
        n: *const c_int,
        a: *mut Complex<c_float>,
        lda: *const c_int,
        ipiv: *const c_int,
        work: *mut Complex<c_float>,
        lwork: *const c_int,
        info: *mut c_int,
    );

    pub fn zgetri_(
        n: *const c_int,
        a: *mut Complex<c_double>,
        lda: *const c_int,
        ipiv: *const c_int,
        work: *mut Complex<c_double>,
        lwork: *const c_int,
        info: *mut c_int,
    );
}
