//! # Hasty
//!
//! Hasty is a Rust wrapper for BLAS libraries, making use of highly
//! optimised BLAS implementations found on the system. It finds these
//! libraries via CMake, which is run as part of the build process.
//!
//! To specify a particular BLAS library, set the `HASTY_BLAS_PATH`
//! environment variable to the absolute path to the library. If this
//! variable is not set, CMake will search for a library on the system.
//!
//! ## BLAS Vendors
//!
//! Hasty supports multiple BLAS vendors, and will search your system
//! for a BLAS library automatically. If you want to specify which
//! library to use, you can set a feature flag in your `Cargo.toml`.
//!
//! - `generic`: Generic reference implementation
//! - `acml`: AMD Core Math Library
//! - `accelerate`: Apple Accelerate
//! - `arm`: ARM Performance Libraries
//! - `atlas`: ATLAS (Automatically Tuned Linear Algebra Software)
//! - `blis`: BLIS/Flame Framework
//! - `openblas`: OpenBLAS
//! - `mkl`: Intel MKL
//!
//! For example:
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! hasty = { "x.y", features = ["openblas"] } # Use OpenBLAS backend
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic, clippy::nursery)]
#![doc(
html_favicon_url = "https://raw.githubusercontent.com/Pencilcaseman/hasty/master/img/logo_dark_mode.png"
)]
#![doc(
html_logo_url = "https://raw.githubusercontent.com/Pencilcaseman/hasty/master/img/logo_dark_mode.png"
)]

pub mod errors;
pub mod hasty_impl;

#[cfg(feature = "blas")]
/// The BLAS wrappers inside Hasty
pub mod blas;

#[cfg(feature = "opencl")]
/// The OpenCL wrappers inside Hasty
pub mod opencl;
