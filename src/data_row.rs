/// A copy of a row of the sounding data.
#[derive(Clone, Default, Copy, Debug, PartialEq)]
pub struct DataRow {
    /// Pressure in hPa
    pub pressure: Option<f64>,
    /// Temperature in C
    pub temperature: Option<f64>,
    /// Wet bulb temperature in C
    pub wet_bulb: Option<f64>,
    /// Dew point in C
    pub dew_point: Option<f64>,
    /// Equivalent potential temperature in Kelvin
    pub theta_e: Option<f64>,
    /// Wind direction (from) in degrees.
    pub direction: Option<f64>,
    /// Wind speed in knots
    pub speed: Option<f64>,
    /// Pressure vertical velocity in Pa/sec
    pub omega: Option<f64>,
    /// Geopotential Height in meters
    pub height: Option<f64>,
    /// Cloud fraction in percent
    pub cloud_fraction: Option<f64>,
}
