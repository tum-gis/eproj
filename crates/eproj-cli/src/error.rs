use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EprojError(#[from] eproj::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}
