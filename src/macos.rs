use std::ffi::{CStr, CString};

use libc::c_void;

use crate::{OsInfo, OsKind};

pub fn get_os_info() -> OsInfo {
    OsInfo {
        kind: OsKind::Macos,
        name: None,
        version: os_version(),
    }
}

fn os_version() -> Option<String> {
    let buf = get_sys_value_by_name(c"kern.osproductversion").ok()?;
    let cstr = CString::from_vec_with_nul(buf).ok()?;
    cstr.into_string().ok()
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
    }
}
