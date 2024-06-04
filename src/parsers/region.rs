use quick_xml::DeError;
use thiserror::Error;

#[derive(Debug)]
pub struct Region {
    pub inner: String,
}

#[derive(Debug, Error)]
pub enum IntoRegionError {
    /// Something bad happened in deserialization.
    #[error("deserialization failed")]
        DeserializationError {
        /// The error source. Look here for what went wrong.
        #[from]
        source: DeError,
    },
}
