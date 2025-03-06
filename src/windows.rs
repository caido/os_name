use windows_registry::LOCAL_MACHINE;
use windows_version::OsVersion;

use crate::{OsInfo, OsKind};

pub fn get_os_info() -> OsInfo {
    let raw = OsVersion::current();

    OsInfo {
        kind: OsKind::Windows,
        name: os_name(&raw),
        version: Some(os_version(&raw)),
    }
}

fn os_version(version: &OsVersion) -> String {
    if is_windows_11(version) {
        "11".to_string()
    } else {
        version.major.to_string()
    }
}

fn os_name(version: &OsVersion) -> Option<String> {
    let registry_key = LOCAL_MACHINE
        .open("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .ok()?;
    let mut value = registry_key.get_string("ProductName").ok()?;

    if is_windows_11(version) && value.starts_with("Windows 10") {
        value.replace_range(9..10, "1");
    }

    Some(value)
}

fn is_windows_11(version: &OsVersion) -> bool {
    // Windows 11 shares dwMajorVersion with Windows 10
    // this workaround tries to disambiguate that by checking
    // if the dwBuildNumber is from Windows 11 releases (>= 22000).
    if version.major > 10 {
        true
    } else if version.major == 10 && version.build >= 22000 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os_info() {
        let os_info = get_os_info();
        assert_eq!(os_info.kind, OsKind::Windows);
        assert!(os_info.version.is_some());
        assert!(os_info.name.is_some());
    }
}
