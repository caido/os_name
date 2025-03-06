# OS Name
[<img alt="github" src="https://img.shields.io/badge/github-caido/os_name-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/caido/os_name)
[<img alt="crates.io" src="https://img.shields.io/crates/v/os_name?color=fc8d62&logo=rust&style=for-the-badge" height="20">](https://crates.io/crates/os_name)

Get OS details on multiple platforms.
The goal of the library is to provide a simple interface to get the OS kind, version and name.

```rust
use os_name::get_os_info;

let os_info = get_os_info();

assert_eq!(os_info.name.unwrap(), "Sonoma");
```

## Features

- `serde`: Enable this feature to add Serde support.
