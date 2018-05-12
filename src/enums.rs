use std::fmt;

/// The names of the profiles which may be stored in a sounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// Pressure in hPa
    Pressure,
    /// Temperature in C
    Temperature,
    /// Wet bulb temperature in C
    WetBulb,
    /// Dew point in C
    DewPoint,
    /// Equivalent potential temperature in Kelvin
    ThetaE,
    /// Wind direction (from) in degrees.
    WindDirection,
    /// Wind speed in knots
    WindSpeed,
    /// Pressure vertical velocity in Pa/sec
    PressureVerticalVelocity,
    /// Geopotential Height in meters
    GeopotentialHeight,
    /// Cloud fraction in percent
    CloudFraction,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Profile::*;
        let string_rep = match *self {
            Pressure => "pressure",
            Temperature => "temperature",
            WetBulb => "wet bulb temperature",
            DewPoint => "dew point temperature",
            ThetaE => "equivalent potential temperature",
            WindDirection => "wind direction",
            WindSpeed => "wind speed",
            PressureVerticalVelocity => "vertical velocity",
            GeopotentialHeight => "height",
            CloudFraction => "cloud fraction",
        };

        write!(f, "{}", string_rep)
    }
}

/// Surface based values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    /// Surface pressure reduce to mean sea level (hPa)
    MSLP,
    /// Surface pressure (hPa)
    StationPressure,
    /// Low cloud fraction
    LowCloud,
    /// Mid cloud fraction
    MidCloud,
    /// Hi cloud fraction
    HighCloud,
    /// Wind Direction in degrees. This is the direction the wind is coming from.
    WindDirection,
    /// Wind speed in knots.
    WindSpeed,
    /// 2 meter temperatures (C)
    Temperature,
    /// 2 meter dew point (C)
    DewPoint,
    /// Precipitation (in)
    Precipitation,
}

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Surface::*;
        let string_rep = match *self {
            MSLP => "sea level pressure",
            StationPressure => "station pressure",
            LowCloud => "low cloud fraction",
            MidCloud => "mid cloud fraction",
            HighCloud => "high cloud fraction",
            WindDirection => "wind direction",
            WindSpeed => "wind speed",
            Temperature => "2-meter temperature",
            DewPoint => "2-meter dew point",
            Precipitation => "precipitation (liquid equivalent)",
        };

        write!(f, "{}", string_rep)
    }
}
