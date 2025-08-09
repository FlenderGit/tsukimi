use thiserror::Error;

use crate::api::ApiError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    WebError(#[from] ApiError),
}
