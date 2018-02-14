#![warn(missing_docs)]
//! Library to represent an atmospheric sounding.

extern crate chrono;
extern crate smallvec;

pub mod sounding;
pub use sounding::{DataRow, Index, Profile, Sounding, StationInfo, Surface};

/// The size used in `smallvec` for the number of items to place on the stack. This is mainly used
/// in derived crates.
pub const SMALL_VEC_SIZE: usize = 2;
