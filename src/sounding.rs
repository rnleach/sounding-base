//! Data type and methods to store an atmospheric sounding.

use chrono::NaiveDateTime;

use error::*;
use missing_value::OptionVal;

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
    pub lat: OptionVal<f64>,
    /// Longitude of grid point used to make sounding.
    pub lon: OptionVal<f64>,
    /// Elevation of grid point in meters, this is in model terrain, not necessarily the same as
    /// the real world.
    pub elevation: OptionVal<f64>,

    // Sounding Indexes
    /// Showalter index
    pub show: OptionVal<f64>,
    /// Lifted index
    pub li: OptionVal<f64>,
    /// Severe Weather Threat Index
    pub swet: OptionVal<f64>,
    /// K-index
    pub kinx: OptionVal<f64>,
    /// Lifting Condensation Level, or LCL (hPa), pressure vertical coordinate.
    pub lclp: OptionVal<f64>,
    /// Precipitable Water (mm)
    pub pwat: OptionVal<f64>,
    /// Total-Totals
    pub totl: OptionVal<f64>,
    /// Convective Available Potential Energy, or CAPE. (J/kg)
    pub cape: OptionVal<f64>,
    /// Temperature at LCL (K)
    pub lclt: OptionVal<f64>,
    /// Convective Inhibitive Energy, or CIN (J/kg)
    pub cins: OptionVal<f64>,
    /// Equilibrium Level (hPa), pressure vertical coordinate
    pub eqlv: OptionVal<f64>,
    /// Level of Free Convection (hPa), pressure vertical coordinate
    pub lfc: OptionVal<f64>,
    /// Bulk Richardson Number
    pub brch: OptionVal<f64>,
    /// Haines Index
    pub hain: OptionVal<i32>,

    // Upper air profile
    /// Pressure (hPa) profile
    pub pressure: Vec<OptionVal<f64>>,
    /// Temperature (c) profile
    pub temperature: Vec<OptionVal<f64>>,
    /// Wet-bulb (c) profile
    pub wet_bulb: Vec<OptionVal<f64>>,
    /// Dew Point (C) profile
    pub dew_point: Vec<OptionVal<f64>>,
    /// Equivalent Potential Temperature (K) profile
    pub theta_e: Vec<OptionVal<f64>>,
    /// Wind direction (degrees) profile
    pub direction: Vec<OptionVal<f64>>,
    /// Wind speed (knots) profile
    pub speed: Vec<OptionVal<f64>>,
    /// Vertical velocity (Pa/sec), pressure vertical coordinate
    pub omega: Vec<OptionVal<f64>>,
    /// Geopotential Height (m) profile
    pub height: Vec<OptionVal<f64>>,
    /// Cloud coverage fraction in percent
    pub cloud_fraction: Vec<OptionVal<f64>>,

    // Surface data
    /// Surface pressure reduce to mean sea level (hPa)
    pub mslp: OptionVal<f64>,
    /// Surface pressure (hPa)
    pub station_pres: OptionVal<f64>,
    /// Low cloud fraction
    pub low_cloud: OptionVal<f64>,
    /// Mid cloud fraction
    pub mid_cloud: OptionVal<f64>,
    /// Hi cloud fraction
    pub hi_cloud: OptionVal<f64>,
    /// U - wind speed (m/s) (West -> East is positive)
    pub uwind: OptionVal<f64>,
    /// V - wind speed (m/s) (South -> North is positive)
    pub vwind: OptionVal<f64>,
}

/// A view of a row of the sounding data. Values are named the same as those in a `Sounding`.
#[derive(Default, Debug)]
#[allow(missing_docs)]
pub struct DataRow {
    pub pressure: OptionVal<f64>,
    pub temperature: OptionVal<f64>,
    pub wet_bulb: OptionVal<f64>,
    pub dew_point: OptionVal<f64>,
    pub theta_e: OptionVal<f64>,
    pub direction: OptionVal<f64>,
    pub speed: OptionVal<f64>,
    pub omega: OptionVal<f64>,
    pub height: OptionVal<f64>,
    pub cloud_fraction: OptionVal<f64>,
}

impl Sounding {
    /// Validates the sounding with some simple sanity checks. For instance, checks that pressure
    /// decreases with height.
    #[cfg_attr(feature = "cargo-clippy", allow(cyclomatic_complexity))]
    pub fn validate(&self) -> Result<()> {

        macro_rules! validate_f64_positive {
            ($var:ident, $err_msg:ident, $var_name:expr) => {
                let opt: Option<f64> = self.$var.into();
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
        let mut pressure_one_level_down = ::std::f64::MAX;
        if let Some(val) = self.station_pres.as_option() {
            pressure_one_level_down = val;
        }
        for pres in &self.pressure {
            if pres.as_option().is_none() {
                continue;
            }
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

        // Check height always increases with height.
        let mut height_one_level_down = ::std::f64::MIN;
        for hght in &self.height {
            if hght.as_option().is_none() {
                continue;
            }
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
            }
            if spd.unwrap() < 0.0 {
                error_msg.push_str(&format!("\nWind speed < 0: {} < 0.0", spd.unwrap()));
            }
        }

        // Check that cloud fraction >= 0
        for cld in &self.cloud_fraction {
            if cld.as_option().is_none() {
                continue;
            }
            if cld.unwrap() < 0.0 {
                error_msg.push_str(&format!("\nCloud fraction < 0: {} < 0.0", cld.unwrap()));
            }

        }

        // Index checks
        validate_f64_positive!(cape, error_msg, "CAPE");
        validate_f64_positive!(pwat, error_msg, "PWAT");

        // Check that cin <= 0
        let opt: Option<f64> = self.cins.into();
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
        validate_f64_positive!(low_cloud, error_msg, "low cloud");
        validate_f64_positive!(mid_cloud, error_msg, "mid cloud");
        validate_f64_positive!(hi_cloud, error_msg, "hi cloud");

        if error_msg == "" {
            Ok(())
        } else {
            error_msg.push('\n');
            Err(Error::from(ErrorKind::ValidationError(error_msg)))
        }
    }

    /// Get a row of data values from this sounding.
    pub fn get_data_row(&self, idx: usize) -> Option<DataRow> {

        macro_rules! copy_to_result {
            ($result:ident, $field:ident, $idx:ident) => {
                match self.$field.get($idx) {
                    None => {},
                    Some(opt_val) => $result.$field = *opt_val,
                }
            };
        }
        
        if self.pressure.len() <= idx {return None;}
        
        let mut result = DataRow::default();

        copy_to_result!(result, pressure, idx);
        copy_to_result!(result, temperature, idx);
        copy_to_result!(result, wet_bulb, idx);
        copy_to_result!(result, dew_point, idx);
        copy_to_result!(result, theta_e, idx);
        copy_to_result!(result, direction, idx);
        copy_to_result!(result, speed, idx);
        copy_to_result!(result, omega, idx);
        copy_to_result!(result, height, idx);
        copy_to_result!(result, cloud_fraction, idx);
        
        Some(result)
    }

    /// Interpolate values from the vertical sounding using pressure as the primary coordinate.
    ///
    /// Returns a `DataRow` struct with interpolated values.
    pub fn interpolate(&self, target_p: f64) -> DataRow {

        macro_rules! linear_interp {
            ($res:ident, $blw_idx:ident, $abv_idx:ident,  $run:ident, $dp:ident, $array:ident) => {
                if self.$array.len() > $abv_idx {
                    let val_below = self.$array[$blw_idx].unwrap();
                    let val_above = self.$array[$abv_idx].unwrap();
                    let rise = val_above - val_below;
                    $res.$array = (val_below + $dp * rise/$run).into();
                }
            };
        }

        let mut result = DataRow::default();
        result.pressure = target_p.into();

        let mut below_idx: usize = 0;
        let mut above_idx: usize = 0;
        for (i, p) in self.pressure.iter().enumerate() {
            if let Some(p) = p.as_option() {
                if p > target_p {
                    below_idx = i;
                }
                if p < target_p {
                    above_idx = i;
                    break;
                }
            }
        }

        if above_idx != 0 {
            let p_below = self.pressure[below_idx].unwrap();
            let p_above = self.pressure[above_idx].unwrap();
            let run = p_above - p_below;
            let dp = target_p - p_below;

            if self.temperature.len() > above_idx {
                let t_below = self.temperature[below_idx];
                let t_above = self.temperature[above_idx];
                if t_below.as_option().is_some() && t_above.as_option().is_some(){
                    let t_below = t_below.unwrap();
                    let t_above = t_above.unwrap();
                    let rise = t_above - t_below;
                    result.temperature = (t_below + dp * rise/run).into();
                }
            }

            linear_interp!(result, below_idx, above_idx, run, dp, wet_bulb);
            linear_interp!(result, below_idx, above_idx, run, dp, dew_point);
            linear_interp!(result, below_idx, above_idx, run, dp, theta_e);
            // FIXME: Account for wrap around, use vector interpolation.
            linear_interp!(result, below_idx, above_idx, run, dp, direction);
            linear_interp!(result, below_idx, above_idx, run, dp, speed);
            linear_interp!(result, below_idx, above_idx, run, dp, omega);
            linear_interp!(result, below_idx, above_idx, run, dp, height);
            linear_interp!(result, below_idx, above_idx, run, dp, cloud_fraction);
        }

        result
    }

    /// Given a target pressure, return the row of data values closest to this one.
    pub fn fetch_nearest_pnt(&self, target_p: f64) -> DataRow {

        let mut idx: usize = 0;
        let mut best_abs_diff: f64 = ::std::f64::MAX;
        for (i, p) in self.pressure.iter().enumerate() {
            if let Some(p) = p.as_option() {
                let abs_diff = (target_p - p).abs();
                if abs_diff < best_abs_diff {
                    best_abs_diff = abs_diff;
                    idx = i;
                }
                if abs_diff > best_abs_diff {
                    break;
                }
            }
        }

        self.get_data_row(idx).unwrap()
    }
}
