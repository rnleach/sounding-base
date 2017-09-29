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
///
#[derive(Default)]
pub struct Sounding {
    // Station info section
    /// station number, USAF number, eg 727730
    pub num: OptionVal<i32>,
    /// Valid time of sounding
    pub valid_time: Option<NaiveDateTime>,
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
    pub pressure: Vec<OptionVal<f32>>,
    /// Temperature (c) profile
    pub temperature: Vec<OptionVal<f32>>,
    /// Wet-bulb (c) profile
    pub wet_bulb: Vec<OptionVal<f32>>,
    /// Dew Point (C) profile
    pub dew_point: Vec<OptionVal<f32>>,
    /// Equivalent Potential Temperature (K) profile
    pub theta_e: Vec<OptionVal<f32>>,
    /// Wind direction (degrees) profile
    pub direction: Vec<OptionVal<f32>>,
    /// Wind speed (knots) profile
    pub speed: Vec<OptionVal<f32>>,
    /// Vertical velocity (Pa/sec), pressure vertical coordinate
    pub omega: Vec<OptionVal<f32>>,
    /// Geopotential Height (m) profile
    pub height: Vec<OptionVal<f32>>,
    /// Cloud coverage fraction in percent
    pub cloud_fraction: Vec<OptionVal<f32>>,

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

impl Sounding {
    /// Validates the sounding with some simple sanity checks. For instance, checks that pressure
    /// decreases with height.
    pub fn validate(&self) -> Result<()> {

        macro_rules! validate_f32_positive {
            ($var:ident, $err_msg:ident, $var_name:expr) => {
                let opt: Option<f32> = self.$var.into();
                if let Some(val) = opt {
                    if val < 0.0 {
                        $err_msg.push_str(&format!("\n{} < 0.0: {}", $var_name, val));
                    }
                }
            };
        }

        macro_rules! validate_vector_len {
            ($var:ident, $len:expr, $err_msg:ident, $var_name:expr) => {
                if !self.$var.is_empty() && self.$var.len() != $len {
                    $err_msg
                        .push_str(&format!("\n{} array has different length than pressure array.",
                                    $var_name));
                }
            };
        }

        let mut error_msg = String::from("");

        // Sounding checks
        if self.pressure.is_empty() {
            error_msg.push_str("\nPressure variable required, none given.");
        }

        let len = self.pressure.len();

        validate_vector_len!(temperature, len, error_msg, "Temperature");
        validate_vector_len!(wet_bulb, len, error_msg, "Wet bulb temperature");
        validate_vector_len!(dew_point, len, error_msg, "Dew point");
        validate_vector_len!(theta_e, len, error_msg, "Theta-e");
        validate_vector_len!(direction, len, error_msg, "Wind direction");
        validate_vector_len!(speed, len, error_msg, "wind speed");
        validate_vector_len!(omega, len, error_msg, "Omega (pressure vertical velocity)");
        validate_vector_len!(height, len, error_msg, "Height");
        validate_vector_len!(cloud_fraction, len, error_msg, "Cloud fraction");

        // Check that pressure always decreases with height and that the station pressure is more
        // than the lowest pressure level in sounding.
        let mut pressure_one_level_down = ::std::f32::MAX;
        if let Some(val) = self.station_pres.as_option() {
            pressure_one_level_down = val;
        }
        for pres in &self.pressure {
            if pres.as_option().is_none() {
                continue;
            } else {
                let pres_val = pres.unwrap();
                if pressure_one_level_down < pres_val {
                    error_msg.push_str(&format!(
                        "\nPressure increasing with height: {} < {}",
                        pressure_one_level_down,
                        pres_val
                    ));
                }
                pressure_one_level_down = pres_val;
            }
        }

        // Check height always increases with height.
        let mut height_one_level_down = ::std::f32::MIN;
        for hght in &self.height {
            if hght.as_option().is_none() {
                continue;
            } else {
                let hght_val = hght.unwrap();
                if height_one_level_down > hght_val {
                    error_msg.push_str(&format!(
                        "\nHeight values decreasing with height: {} < {}",
                        height_one_level_down,
                        hght_val
                    ));
                }
                height_one_level_down = hght_val;
            }
        }

        // Check that dew point <= wet bulb <= t
        for (t, wb) in self.temperature.iter().zip(self.wet_bulb.iter()) {
            if t.as_option().is_none() || wb.as_option().is_none() {
                continue;
            }
            if t.unwrap() < wb.unwrap() {
                error_msg.push_str(&format!(
                    "\nTemperature < Wet bulb: {} < {}",
                    t.unwrap(),
                    wb.unwrap()
                ));
            }
        }
        for (t, dp) in self.temperature.iter().zip(self.dew_point.iter()) {
            if t.as_option().is_none() || dp.as_option().is_none() {
                continue;
            }
            if t.unwrap() < dp.unwrap() {
                error_msg.push_str(&format!(
                    "\nTemperature < Dew Point: {} < {}",
                    t.unwrap(),
                    dp.unwrap()
                ));
            }
        }
        for (wb, dp) in self.wet_bulb.iter().zip(self.dew_point.iter()) {
            if wb.as_option().is_none() || dp.as_option().is_none() {
                continue;
            }
            if wb.unwrap() < dp.unwrap() {
                error_msg.push_str(&format!(
                    "\nWet bulb < Dew Point: {} < {}",
                    wb.unwrap(),
                    dp.unwrap()
                ));
            }
        }

        // Check that speed >= 0
        for spd in &self.speed {
            if spd.as_option().is_none() {
                continue;
            } else {
                if spd.unwrap() < 0.0 {
                    error_msg.push_str(&format!("\nWind speed < 0: {} < 0.0", spd.unwrap()));
                }
            }
        }

        // Check that cloud fraction >= 0
        for cld in &self.cloud_fraction {
            if cld.as_option().is_none() {
                continue;
            } else {
                if cld.unwrap() < 0.0 {
                    error_msg.push_str(&format!("\nCloud fraction < 0: {} < 0.0", cld.unwrap()));
                }
            }
        }

        // Index checks
        validate_f32_positive!(cape, error_msg, "CAPE");
        validate_f32_positive!(pwat, error_msg, "PWAT");

        // Check that cin <= 0
        let opt: Option<f32> = self.cins.into();
        if let Some(val) = opt {
            if val > 0.0 {
                error_msg.push_str(&format!("\nCINS > 0.0: {}", val));
            }
        }

        // Check Haines Index = 2, 3, 4, 5, or 6
        let opt: Option<i32> = self.hain.into();
        if let Some(val) = opt {

            match val {
                2...6 => {} // Good values, do nothing.
                _ => error_msg.push_str(&format!("\nInvalid Haines Index: {}", val)),
            }
        }

        // Surface checks
        // Check that hi, mid, and low cloud are all positive or zero
        validate_f32_positive!(low_cloud, error_msg, "low cloud");
        validate_f32_positive!(mid_cloud, error_msg, "mid cloud");
        validate_f32_positive!(hi_cloud, error_msg, "hi cloud");

        if error_msg == "" {
            Ok(())
        } else {
            error_msg.push('\n');
            Err(Error::from(ErrorKind::ValidationError(error_msg)))
        }
    }
}
