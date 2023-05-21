mod error;

pub use crate::error::ResolutionError;

#[cfg(target_os = "linux")]
use xrandr::XHandle;

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;

#[cfg(target_os = "linux")]
pub fn current_resolution() -> Result<(i32, i32), ResolutionError> {
    let monitors = XHandle::open()
        .or_else(|err| Err(err).into())?
        .monitors()
        .or_else(|err| Err(err).into())?;
    monitors
        .iter()
        .find_map(|monitor| {
            if monitor.is_primary {
                Some((monitor.width_px, monitor.height_px))
            } else {
                None
            }
        })
        .ok_or_else(|| ResolutionError::CouldNotGetResolution)
}

#[cfg(target_os = "macos")]
pub fn current_resolution() -> Result<(i32, i32), ResolutionError> {
    let display = CGDisplay::main();
    let width = display.pixels_wide();
    let height = display.pixels_high();
    Ok((width as i32, height as i32))
}
