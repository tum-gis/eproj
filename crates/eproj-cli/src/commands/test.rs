use crate::error::Error;
use eproj::{Coordinate3, Projector, SpatialReferenceIdentifier};

pub fn run() -> Result<(), Error> {
    let projection = Projector::new(
        SpatialReferenceIdentifier::Epsg25832,
        SpatialReferenceIdentifier::Epsg4979,
    )?;

    let result = projection.convert(Coordinate3::new(1.0, 2.0, 3.0))?;

    println!("result: {result:?}");

    Ok(())
}
