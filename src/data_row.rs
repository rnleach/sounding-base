use optional::Optioned;

/// A copy of a row of the sounding data.
#[derive(Clone, Default, Copy, Debug, PartialEq)]
pub struct DataRow {
    /// Pressure in hPa
    pub pressure: Optioned<f64>,
    /// Temperature in C
    pub temperature: Optioned<f64>,
    /// Wet bulb temperature in C
    pub wet_bulb: Optioned<f64>,
    /// Dew point in C
    pub dew_point: Optioned<f64>,
    /// Equivalent potential temperature in Kelvin
    pub theta_e: Optioned<f64>,
    /// Wind direction (from) in degrees.
    pub direction: Optioned<f64>,
    /// Wind speed in knots
    pub speed: Optioned<f64>,
    /// Pressure vertical velocity in Pa/sec
    pub omega: Optioned<f64>,
    /// Geopotential Height in meters
    pub height: Optioned<f64>,
    /// Cloud fraction in percent
    pub cloud_fraction: Optioned<f64>,
}
