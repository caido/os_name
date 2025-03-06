use std::{fs::File, io::Read};

use crate::{OsInfo, OsKind};

pub fn get_os_info() -> OsInfo {
    let (name, version) = if let Some(raw) = raw() {
        (raw.name, raw.version)
    } else {
        (None, None)
    };

    OsInfo {
        kind: OsKind::Linux,
        name,
        version,
    }
}

struct OsInfoRaw {
    name: Option<String>,
    version: Option<String>,
}

fn raw() -> Option<OsInfoRaw> {
    let data = File::open("/etc/os-release")
        .and_then(|mut f| {
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;
            Ok(buf)
        })
        .ok()?;

    let mut name = None;
    let mut version = None;
    for line in data.lines() {
        if let Some(s) = line.strip_prefix("NAME=") {
            name = Some(s.replace('"', ""))
        } else if let Some(s) = line.strip_prefix("VERSION_ID=") {
            version = Some(s.replace('"', ""))
        }
    }

    Some(OsInfoRaw { name, version })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os_info() {
        let os_info = get_os_info();
        assert_eq!(os_info.kind, OsKind::Linux);
        assert!(os_info.version.is_some());
        assert!(os_info.name.is_some());
    }
}
