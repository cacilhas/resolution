mod error;

pub use crate::error::ResolutionError;

#[cfg(target_os = "linux")]
use xrandr::XHandle;

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;

#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;

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

/// [Windows] Returns the resolution of the main display using a call to the `GetSystemMetrics` function of the Win32 API through the `windows-sys` crate.
/// For more information about the `GetSystemMetrics` function, see Microsoft's official documentation: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics
#[cfg(target_os = "windows")]
pub fn current_resolution() -> Result<(i32, i32), ResolutionError> {
    let width = unsafe { GetSystemMetrics(0) };
    let height = unsafe { GetSystemMetrics(1) };
    Ok((width, height))
}

// Defaults to error

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub fn current_resolution() -> Result<(i32, i32), ResolutionError> {
    Err(ResolutionError::NotImplemented)
}
