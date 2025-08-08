use crate::Coordinate3;
use crate::bindings::proj::{Coord, Proj};
use crate::error::Error;
use crate::srid::SpatialReferenceIdentifier;
use nalgebra::{Isometry3, Point3, Translation3, UnitQuaternion, Vector3};

#[derive(Debug, Clone)]
pub struct Projector {
    from: SpatialReferenceIdentifier,
    to: SpatialReferenceIdentifier,
    proj: Proj,
}

impl Projector {
    pub fn new(
        from: SpatialReferenceIdentifier,
        to: SpatialReferenceIdentifier,
    ) -> Result<Self, Error> {
        let proj = Proj::new_known_crs(from.as_str(), to.as_str(), None)?;

        let projector = Self { from, to, proj };
        Ok(projector)
    }

    pub fn convert(&self, point: Coordinate3) -> Result<Coordinate3, Error> {
        let result = self.proj.convert((point.x(), point.y(), point.z()))?;
        let resulting_point = Coordinate3::new(result.x(), result.y(), result.z());

        Ok(resulting_point)
    }

    pub fn convert_point(&self, point: Point3<f64>) -> Result<Point3<f64>, Error> {
        let result = self.proj.convert((point.x, point.y, point.z))?;
        let resulting_point = Point3::new(result.x(), result.y(), result.z());

        Ok(resulting_point)
    }

    pub fn convert_points(&self, points: Vec<Point3<f64>>) -> Result<Vec<Point3<f64>>, Error> {
        let mut proj_points: Vec<(f64, f64, f64)> =
            points.into_iter().map(|p| (p.x, p.y, p.z)).collect();
        let result = self.proj.convert_array(&mut proj_points)?;
        let resulting_points: Vec<Point3<f64>> = result
            .iter_mut()
            .map(|p| Point3::new(p.x(), p.y(), p.z()))
            .collect();

        Ok(resulting_points)
    }

    pub fn convert_isometry(&self, isometry: Isometry3<f64>) -> Result<Isometry3<f64>, Error> {
        let converted_point: Point3<f64> =
            self.convert_point(isometry.translation.vector.into())?;
        let x_axis: Point3<f64> = isometry * Point3::new(1.0, 0.0, 0.0);
        let y_axis: Point3<f64> = isometry * Point3::new(0.0, 1.0, 0.0);
        // let z_axis: Point3<f64> = isometry * Point3::new(0.0, 0.0, 1.0);
        let converted_x_axis: Vector3<f64> =
            self.convert_point(x_axis)?.coords - converted_point.coords;
        let converted_y_axis: Vector3<f64> =
            self.convert_point(y_axis)?.coords - converted_point.coords;
        // let converted_z_axis: Vector3<f64> =
        //    self.convert_point(z_axis)?.coords - converted_point.coords;

        let translation =
            Translation3::new(converted_point.x, converted_point.y, converted_point.z);
        let rotation = nalgebra::Rotation3::from_basis_unchecked(&[
            converted_x_axis,
            converted_y_axis,
            converted_x_axis.cross(&converted_y_axis),
        ]);
        // rotation.renormalize();
        let isometry =
            Isometry3::from_parts(translation, UnitQuaternion::from_rotation_matrix(&rotation));
        Ok(isometry)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Coordinate3, Projector, SpatialReferenceIdentifier};
    use nalgebra::{Isometry3, Point3, UnitQuaternion};
    use std::f64::consts::PI;

    #[test]
    fn it_works() {
        let projection = Projector::new(
            SpatialReferenceIdentifier::Epsg25832,
            SpatialReferenceIdentifier::Epsg4978,
        )
        .unwrap();

        let _result = projection
            .convert(Coordinate3::new(691045.828, 5336014.506, 534.671))
            .unwrap();

        // assert_eq!(result, 4);
    }

    #[test]
    fn test_projection_point() {
        let projection = Projector::new(
            SpatialReferenceIdentifier::Epsg4978,
            SpatialReferenceIdentifier::Epsg4979,
        )
        .unwrap();

        let result = projection
            .convert(Coordinate3::new(
                4177139.6424619956,
                855008.837066541,
                4728267.313036945,
            ))
            .unwrap();
    }

    #[test]
    fn test_projection_points() {
        let projection = Projector::new(
            SpatialReferenceIdentifier::Epsg4978,
            SpatialReferenceIdentifier::Epsg4979,
        )
        .unwrap();

        let points = vec![
            Point3::new(4177139.6424619956, 855008.837066541, 4728267.313036945),
            Point3::new(4177139.6424619956, 855008.837066541, 4728267.313036945),
        ];

        let result = projection.convert_points(points).unwrap();
        println!("Result: {result:?}");
    }

    #[test]
    fn test_convert_isometry() {
        let projection = Projector::new(
            SpatialReferenceIdentifier::Epsg25832,
            SpatialReferenceIdentifier::Epsg4326,
        )
        .unwrap();

        let isometry = Isometry3::from_parts(
            Point3::new(691045.828, 5336014.506, 534.671).into(),
            UnitQuaternion::from_euler_angles(0.0, 0.0, PI),
        );
        println!("isometry {:?}", isometry.rotation.euler_angles());

        let converted_isometry = projection.convert_isometry(isometry).unwrap();
        println!(
            "converted_isometry {:?}",
            converted_isometry.rotation.euler_angles()
        );
        let _affine = isometry.to_matrix();

        // assert_eq!(result, 4);
    }
}
