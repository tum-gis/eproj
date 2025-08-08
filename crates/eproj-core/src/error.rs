use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ProjCreateError(#[from] crate::bindings::proj::ProjCreateError),
    #[error(transparent)]
    ProjCreatError(#[from] crate::bindings::proj::ProjError),

    #[error("Unknown EPSG code")]
    UnknownEpsgCodeError(),
}
