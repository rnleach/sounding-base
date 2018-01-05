#![warn(missing_docs)]
//! Library to represent an atmospheric sounding.

extern crate chrono;

pub mod sounding;
pub use sounding::{DataRow, Index, Profile, Sounding, Surface};
