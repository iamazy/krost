pub mod record;
pub mod render;
pub mod schema;
pub mod types;
pub mod util;

use thiserror::Error;

pub trait Krost: KrostType {
    fn version_added() -> Option<i16>;
    fn version_removed() -> Option<i16>;
    fn apikey() -> Option<i16>;
}

#[derive(Error, Debug)]
pub enum KrostError {
    #[error("Cannot read/write data: {0}")]
    IO(#[from] std::io::Error),

    #[error("Overflow converting integer: {0}")]
    Overflow(#[from] std::num::TryFromIntError),

    #[error("Malformed data: {0}")]
    Malformed(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Invalid version: min={min:?}, max={max:?}, version={version:?}")]
    InvalidVersion {
        min: Option<i16>,
        max: Option<i16>,
        version: i16,
    },
}

pub trait KrostType: Sized {
    fn decode<D: std::io::Read>(buf: &mut D, version: i16) -> Result<Self, KrostError>;
    fn encode<E: std::io::Write>(&self, buf: &mut E, version: i16) -> Result<usize, KrostError>;
}
