//! `eproj` is a library for coordinate reference system transformations.
//!
//! It wraps the crate [proj-sys](https://github.com/georust/proj/tree/main/proj-sys) and
//! provides full 3D coordinate projection support. The 3D support was developed for the
//! [tyler](https://github.com/3DGI/tyler) crate and further extended.

pub use eproj_core::{Coordinate3, Error, Projector, SpatialReferenceIdentifier};
