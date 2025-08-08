mod error;
mod projection;
mod srid;

mod bindings;
mod coordinate;

#[doc(inline)]
pub use error::Error;

#[doc(inline)]
pub use srid::SpatialReferenceIdentifier;

#[doc(inline)]
pub use coordinate::Coordinate3;

#[doc(inline)]
pub use projection::Projector;
