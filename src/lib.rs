//! Traits that map LAPACK accelerated types (e.g. `f32`) to their respective LAPACK calls (e.g.
//! `dgetri`)
//!
//! This library provides no safe wrappers around the foreign LAPACK functions.
//!
//! If you are looking for a LAPACK accelerated linear algebra, check out [linalg].
//!
//! [linalg]: https://github.com/japaric/linalg.rs

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(libc)]

extern crate libc;
extern crate complex;

pub mod getrf;
pub mod getri;

mod ffi;

/// Types with `getri` acceleration
pub trait Getri {
    /// Returns the foreign `getri`
    fn getri() -> getri::Fn<Self>;
}

/// Types with `getrf` acceleration
pub trait Getrf {
    /// Returns the foreign `getrf`
    fn getrf() -> getrf::Fn<Self>;
}
