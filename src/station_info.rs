/// Station information including location data and identification number.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StationInfo {
    /// station number, USAF number, eg 727730
    num: Option<i32>,
    /// Latitude and longitude.
    location: Option<(f64, f64)>,
    /// Elevation in meters, this may be in model terrain, not necessarily the same as
    /// the real world.
    elevation: Option<f64>,
}

impl StationInfo {
    /// Create a new `StationInfo` object.
    #[inline]
    pub fn new_with_values<T, U, V>(station_num: T, location: U, elevation: V) -> Self
    where
        T: Into<Option<i32>>,
        U: Into<Option<(f64, f64)>>,
        V: Into<Option<f64>>,
    {
        StationInfo {
            num: station_num.into(),
            location: location.into(),
            elevation: elevation.into(),
        }
    }

    /// Create a new object with default values.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder method to add a station number.
    #[inline]
    pub fn with_station<T>(mut self, number: T) -> Self
    where
        Option<i32>: From<T>,
    {
        self.num = Option::from(number);

        self
    }

    /// Builder method to add a location.
    #[inline]
    pub fn with_lat_lon<T>(mut self, coords: T) -> Self
    where
        Option<(f64, f64)>: From<T>,
    {
        self.location = Option::from(coords);
        self
    }

    /// Builder method to add elevation.
    #[inline]
    pub fn with_elevation<T>(mut self, elev: T) -> Self
    where
        Option<f64>: From<T>,
    {
        self.elevation = Option::from(elev);
        self
    }

    /// station number, USAF number, eg 727730
    #[inline]
    pub fn station_num(&self) -> Option<i32> {
        self.num
    }

    /// Latitude and longitude.
    #[inline]
    pub fn location(&self) -> Option<(f64, f64)> {
        self.location
    }

    /// Elevation in meters, this may be in model terrain, not necessarily the same as
    /// the real world.
    #[inline]
    pub fn elevation(&self) -> Option<f64> {
        self.elevation
    }
}
