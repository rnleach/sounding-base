#![warn(missing_docs)]
//! Library to represent an atmospheric sounding.

extern crate chrono;

pub mod missing_value;
pub use missing_value::{MissingData, OptionVal};

pub mod sounding;
pub use sounding::{DataRow, Index, Profile, Sounding, Surface};
