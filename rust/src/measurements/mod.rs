//! Various implementations of Measurement.
//!
//! The different [`Measurement`] implementations in this module are accessed by calling the appropriate constructor function.
//! Constructors are named in the form `make_xxx()`, where `xxx` indicates what the resulting `Measurement` does.
#[cfg(all(feature="use-mpfr", feature="contrib"))]
pub mod discrete_gaussian;
#[cfg(all(feature="use-mpfr", feature="contrib"))]
pub use crate::measurements::discrete_gaussian::*;

#[cfg(feature="contrib")]
pub mod discrete_laplace;
#[cfg(feature="contrib")]
pub use crate::measurements::discrete_laplace::*;

#[cfg(all(feature="floating-point", feature="contrib"))]
pub mod laplace;
#[cfg(all(feature="floating-point", feature="contrib"))]
pub use crate::measurements::laplace::*;

#[cfg(all(feature="floating-point", feature="contrib", feature="use-mpfr"))]
pub mod gaussian;
#[cfg(all(feature="floating-point", feature="contrib", feature="use-mpfr"))]
pub use crate::measurements::gaussian::*;

#[cfg(all(feature="floating-point", feature="contrib"))]
pub mod ptr;
#[cfg(all(feature="floating-point", feature="contrib"))]
pub use crate::measurements::ptr::*;

#[cfg(feature="contrib")]
pub mod randomized_response;
#[cfg(feature="contrib")]
pub use crate::measurements::randomized_response::*;

#[cfg(all(feature="use-mpfr", feature="floating-point", feature="contrib"))]
pub mod alp;
#[cfg(all(feature="use-mpfr", feature="floating-point", feature="contrib"))]
pub use crate::measurements::alp::*;