#![warn(missing_docs)]
//! Library to represent an atmospheric sounding.

#![recursion_limit = "1024"]
extern crate chrono;
use chrono::NaiveDateTime;

#[macro_use]
extern crate error_chain;

pub mod error;
pub use error::*;

pub mod missing_value;
pub use missing_value::{OptionVal, MissingData};

/// All the variables stored in the sounding.
///
/// The upper air profile variables are stored in parallel vectors. If a profile lacks a certain
/// variable, e.g. cloud fraction, that whole vector has length 0 instead of being full of missing
/// values.
pub struct Sounding {
    // Station info section
    /// station number, USAF number, eg 727730
    pub num: OptionVal<i32>,
    /// Valid time of sounding
    pub valid_time: NaiveDateTime,
    /// Difference in model initialization time and `valid_time` in hours.
    pub lead_time: OptionVal<i32>,
    /// Latitude of grid point used to make sounding.
    pub lat: OptionVal<f32>,
    /// Longitude of grid point used to make sounding.
    pub lon: OptionVal<f32>,
    /// Elevation of grid point in meters, this is in model terrain, not necessarily the same as
    /// the real world.
    pub elevation: OptionVal<f32>,

    // Sounding Indexes
    /// Showalter index
    pub show: OptionVal<f32>,
    /// Lifted index
    pub li: OptionVal<f32>,
    /// Severe Weather Threat Index
    pub swet: OptionVal<f32>,
    /// K-index
    pub kinx: OptionVal<f32>,
    /// Lifting Condensation Level, or LCL (hPa), pressure vertical coordinate.
    pub lclp: OptionVal<f32>,
    /// Precipitable Water (mm)
    pub pwat: OptionVal<f32>,
    /// Total-Totals
    pub totl: OptionVal<f32>,
    /// Convective Available Potential Energy, or CAPE. (J/kg)
    pub cape: OptionVal<f32>,
    /// Temperature at LCL (K)
    pub lclt: OptionVal<f32>,
    /// Convective Inhibitive Energy, or CIN (J/kg)
    pub cins: OptionVal<f32>,
    /// Equilibrium Level (hPa), pressure vertical coordinate
    pub eqlv: OptionVal<f32>,
    /// Level of Free Convection (hPa), pressure vertical coordinate
    pub lfc: OptionVal<f32>,
    /// Bulk Richardson Number
    pub brch: OptionVal<f32>,
    /// Haines Index
    pub hain: OptionVal<i32>,

    // Upper air profile
    /// Pressure (hPa) profile
    pub pressure: Vec<f32>,
    /// Temperature (c) profile
    pub temperature: Vec<f32>,
    /// Wet-bulb (c) profile
    pub wet_bulb: Vec<f32>,
    /// Dew Point (C) profile
    pub dew_point: Vec<f32>,
    /// Equivalent Potential Temperature (K) profile
    pub theta_e: Vec<f32>,
    /// Wind direction (degrees) profile
    pub direction: Vec<f32>,
    /// Wind speed (knots) profile
    pub speed: Vec<f32>,
    /// Vertical velocity (Pa/sec), pressure vertical coordinate
    pub omega: Vec<f32>,
    /// Geopotential Height (m) profile
    pub height: Vec<f32>,
    /// Cloud coverage fraction in percent
    pub cloud_fraction: Vec<f32>,

    // Surface data
    /// Surface pressure reduce to mean sea level (hPa)
    pub mslp: OptionVal<f32>,
    /// Surface pressure (hPa)
    pub station_pres: OptionVal<f32>,
    /// Low cloud fraction
    pub low_cloud: OptionVal<f32>,
    /// Mid cloud fraction
    pub mid_cloud: OptionVal<f32>,
    /// Hi cloud fraction
    pub hi_cloud: OptionVal<f32>,
    /// U - wind speed (m/s) (West -> East is positive)
    pub uwind: OptionVal<f32>,
    /// V - wind speed (m/s) (South -> North is positive)
    pub vwind: OptionVal<f32>,
}
