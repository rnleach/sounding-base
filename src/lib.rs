#![deny(missing_docs)]
//! Library to represent an atmospheric sounding with pressure as the vertical coordinate.

//
// API
//

pub use sounding::{DataRow, Profile, Sounding, StationInfo, Surface};

//
// Internal use only
//

extern crate chrono;
extern crate smallvec;

extern crate metfor;

mod sounding;
