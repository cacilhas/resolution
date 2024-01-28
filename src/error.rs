use std::{fmt, write};

#[derive(Debug, thiserror::Error)]
pub enum ResolutionError {
    CouldNotGetResolution,
    NotImplemented,
    #[cfg(target_os = "linux")]
    XrandrError(xrandr::XrandrError),
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolutionError::CouldNotGetResolution => write!(f, "failed to get screen resolution"),
            #[cfg(target_os = "linux")]
            ResolutionError::NotImplemented => {
                write!(f, "resolution not implemented for the current O.S.")
            }
            ResolutionError::XrandrError(err) => {
                write!(f, "error from xrandr: {err:?}")
            }
        }
    }
}

#[cfg(target_os = "linux")]
impl From<xrandr::XrandrError> for ResolutionError {
    fn from(value: xrandr::XrandrError) -> Self {
        Self::XrandrError(value)
    }
}
