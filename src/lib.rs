#![warn(missing_docs)]
//! Library to represent an atmospheric sounding.

#![recursion_limit = "1024"]
extern crate chrono;
use chrono::NaiveDateTime;

#[macro_use]
extern crate error_chain;

pub mod error;

/// All the variables stored in the sounding.
///
/// Missing values are -9999.0. The upper air profile variables are stored in parallel vectors.
/// If a profile lacks a certain variable, e.g. cloud fraction, that whole vector has length 0
/// instead of being full of missing values.
pub struct Sounding {
    // Station info section
    /// station number, USAF number, eg 727730
    pub num: i32,
    /// Valid time of sounding
    pub valid_time: NaiveDateTime,
    /// Difference in model initialization time and `valid_time` in hours.
    pub lead_time: i32,
    /// Latitude of grid point used to make sounding.
    pub lat: f32,
    /// Longitude of grid point used to make sounding.
    pub lon: f32,
    /// Elevation of grid point in meters, this is in model terrain, not necessarily the same as
    /// the real world.
    pub elevation: f32,

    // Sounding Indexes
    /// Showalter index
    pub show: f32,
    /// Lifted index
    pub li: f32,
    /// Severe Weather Threat Index
    pub swet: f32,
    /// K-index
    pub kinx: f32,
    /// Lifting Condensation Level, or LCL (hPa), pressure vertical coordinate.
    pub lclp: f32,
    /// Precipitable Water (mm)
    pub pwat: f32,
    /// Total-Totals
    pub totl: f32,
    /// Convective Available Potential Energy, or CAPE. (J/kg)
    pub cape: f32,
    /// Temperature at LCL (K)
    pub lclt: f32,
    /// Convective Inhibitive Energy, or CIN (J/kg)
    pub cins: f32,
    /// Equilibrium Level (hPa), pressure vertical coordinate
    pub eqlv: f32,
    /// Level of Free Convection (hPa), pressure vertical coordinate
    pub lfc: f32,
    /// Bulk Richardson Number
    pub brch: f32,

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
    pub mslp: f32,
    /// Surface pressure (hPa)
    pub station_pres: f32,
    /// Low cloud fraction
    pub low_cloud: f32,
    /// Mid cloud fraction
    pub mid_cloud: f32,
    /// Hi cloud fraction
    pub hi_cloud: f32,
    /// U - wind speed (m/s) (West -> East is positive)
    pub uwind: f32,
    /// V - wind speed (m/s) (South -> North is positive)
    pub vwind: f32,
}
