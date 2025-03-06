#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
pub use self::macos::get_os_info;

#[cfg(target_os = "windows")]
pub use self::windows::get_os_info;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsKind {
    Linux,
    Macos,
    Windows,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct OsInfo {
    pub kind: OsKind,
    pub name: Option<String>,
    pub version: Option<String>,
}
