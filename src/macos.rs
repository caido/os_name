use std::ffi::{CStr, CString};

use libc::c_void;

use crate::{OsInfo, OsKind};

pub fn get_os_info() -> OsInfo {
    let raw = raw();

    OsInfo {
        kind: OsKind::Macos,
        name: os_name(&raw),
        version: os_version(&raw),
    }
}

fn os_version(version: &Option<OsVersion>) -> Option<String> {
    let version = version.as_ref()?;
    if version.major > 10 {
        Some(format!("{}", version.major))
    } else {
        Some(format!("{}.{}", version.major, version.minor.unwrap_or(0)))
    }
}

fn os_name(version: &Option<OsVersion>) -> Option<String> {
    let version = version.as_ref()?;

    let name = match (version.major, version.minor) {
        (15, _) => "Sequoia",
        (14, _) => "Sonoma",
        (13, _) => "Ventura",
        (12, _) => "Monterey",
        (11, _) => "Big Sur",
        (10, Some(16)) => "Big Sur",
        (10, Some(15)) => "Catalina",
        (10, Some(14)) => "Mojave",
        (10, Some(13)) => "High Sierra",
        (10, Some(12)) => "Sierra",
        _ => return None,
    };

    Some(name.to_string())
}

struct OsVersion {
    major: u8,
    minor: Option<u8>,
}

fn raw() -> Option<OsVersion> {
    // Get version
    let buf = get_sys_value_by_name(c"kern.osproductversion").ok()?;
    let cstr = CString::from_vec_with_nul(buf).ok()?;
    let version = cstr.into_string().ok()?;

    // Parse
    let mut version = version.split(".");
    let major = version.next()?.parse().ok()?;
    let minor = version.next().and_then(|s| s.parse().ok());

    Some(OsVersion { major, minor })
}

fn get_sys_value_by_name(name: &CStr) -> Result<Vec<u8>, ()> {
    let mut len = 0;
    // Get value length
    unsafe {
        // SAFETY: `sysctlbyname` returns 0 on success
        let res = libc::sysctlbyname(
            name.as_ptr() as *const _,
            std::ptr::null_mut(),
            &mut len,
            std::ptr::null_mut(),
            0,
        );
        if res != 0 {
            return Err(());
        }
    }

    // Get value
    let mut buf = vec![0_u8; len as _];
    unsafe {
        // SAFETY: `sysctlbyname` returns 0 on success
        let res = libc::sysctlbyname(
            name.as_ptr() as *const _,
            buf.as_mut_ptr() as *mut c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        );
        if res != 0 {
            return Err(());
        }
    }
    buf.resize(len, 0);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os_info() {
        let os_info = get_os_info();
        assert_eq!(os_info.kind, OsKind::Macos);
        assert!(os_info.version.is_some());
        assert!(os_info.name.is_some());
    }
}
