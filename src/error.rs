macro_rules! from_err {
    ($type:ty > $variant:ident) => {
        impl From<$type> for KubeError {
            fn from(s: $type) -> Self {
                KubeError::$variant(s)
            }
        }
    };
}

/// Result type used throughout the library.
pub type KubeResult<T> = Result<T, KubeError>;

/// Error type used throughout the library.
#[derive(Debug, Fail)]
pub enum KubeError {
    #[fail(display = "JSON coding error: {}", _0)]
    Json(serde_json::Error),
    #[fail(display = "I/O error: {}", _0)]
    Io(#[cause] std::io::Error),
}

from_err!(serde_json::Error > Json);
from_err!(std::io::Error > Io);
