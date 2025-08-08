use std::ops::{Add, Sub};

/// Implemented with [Coord3D](https://github.com/busstoptaktik/geodesy/blob/main/src/coordinate/coor3d.rs)
/// from the `geodesy` crate as a role model.
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Coordinate3 {
    pub coords: [f64; 3],
}

impl Coordinate3 {
    /// A `Coordinate3` from longitude/latitude/height, with the angular input in radians
    pub fn new(first: f64, second: f64, third: f64) -> Self {
        Self {
            coords: [first, second, third],
        }
    }

    /// A `Coordinate3` from latitude/longitude/height, with the angular input in degrees
    pub fn geo(latitude: f64, longitude: f64, height: f64) -> Self {
        Self {
            coords: [longitude.to_radians(), latitude.to_radians(), height],
        }
    }

    /// A `Coordinate3` from longitude/latitude/height, with the angular input in degrees
    pub fn gis(longitude: f64, latitude: f64, height: f64) -> Self {
        Self {
            coords: [longitude.to_radians(), latitude.to_radians(), height],
        }
    }
}

impl Coordinate3 {
    pub fn x(&self) -> f64 {
        self.coords[0]
    }
    pub fn y(&self) -> f64 {
        self.coords[1]
    }
    pub fn z(&self) -> f64 {
        self.coords[2]
    }
}

impl Add for Coordinate3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.coords[0] + other.coords[0],
            self.coords[1] + other.coords[1],
            self.coords[2] + other.coords[2],
        )
    }
}

impl Add<&Coordinate3> for Coordinate3 {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        Self::new(
            self.coords[0] + other.coords[0],
            self.coords[1] + other.coords[1],
            self.coords[2] + other.coords[2],
        )
    }
}

impl Sub for Coordinate3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.coords[0] - other.coords[0],
            self.coords[1] - other.coords[1],
            self.coords[2] - other.coords[2],
        )
    }
}

impl Coordinate3 {
    /// Transform the first two elements of a `Coordinate3` from degrees to radians
    pub fn to_radians(self) -> Self {
        Self::new(
            self.coords[0].to_radians(),
            self.coords[1].to_radians(),
            self.coords[2],
        )
    }

    /// Transform the first two elements of a `Coordinate3` from radians to degrees
    pub fn to_degrees(self) -> Self {
        Self::new(
            self.coords[0].to_degrees(),
            self.coords[1].to_degrees(),
            self.coords[2],
        )
    }

    /// Transform the first two elements of a `Coordinate3` from radians to seconds
    /// of arc.
    pub fn to_arcsec(self) -> Self {
        Self::new(
            self.coords[0].to_degrees() * 3600.,
            self.coords[1].to_degrees() * 3600.,
            self.coords[2],
        )
    }

    /// Transform the internal lon/lat/h/t-in-radians to lat/lon/h/t-in-degrees
    fn to_geo(self) -> Self {
        Self::new(
            self.coords[1].to_degrees(),
            self.coords[0].to_degrees(),
            self.coords[2],
        )
    }
}

impl Coordinate3 {
    /// Euclidean distance between two points in the 3D space.
    pub fn hypot3(&self, other: &Self) -> f64 {
        (self.coords[0] - other.coords[0])
            .hypot(self.coords[1] - other.coords[1])
            .hypot(self.coords[2] - other.coords[2])
    }
}

impl From<Coordinate3> for nalgebra::Point3<f64> {
    fn from(item: Coordinate3) -> Self {
        Self::new(item.x(), item.y(), item.z())
    }
}

impl From<nalgebra::Point3<f64>> for Coordinate3 {
    fn from(item: nalgebra::Point3<f64>) -> Self {
        Self::new(item.x, item.y, item.z)
    }
}

impl From<Coordinate3> for nalgebra::Vector3<f64> {
    fn from(item: Coordinate3) -> Self {
        Self::new(item.x(), item.y(), item.z())
    }
}

impl From<nalgebra::Vector3<f64>> for Coordinate3 {
    fn from(item: nalgebra::Vector3<f64>) -> Self {
        Self::new(item.x, item.y, item.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::Coordinate3;

    #[test]
    fn basic_conversion() {
        let point = nalgebra::Point3::origin();
        let coordinate: Coordinate3 = point.into();

        assert_eq!(coordinate, Coordinate3::new(0.0, 0.0, 0.0));
    }
}
