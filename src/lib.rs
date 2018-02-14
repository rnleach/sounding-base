#![warn(missing_docs)]
//! Library to represent an atmospheric sounding.

extern crate chrono;
extern crate smallvec;

pub mod sounding;
pub use sounding::{DataRow, Index, Profile, Sounding, StationInfo, Surface};
