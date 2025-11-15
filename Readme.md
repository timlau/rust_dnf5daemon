# Rust module to demo use of dnf5 dbus API from Rust.

The code and is under heavy development and is not meant to be used as is
But can be used for inspiration on how to use the dnf5 dbus API

(C) 2025 Tim Lauridsen

License: MIT

## How to run
The module contains a simple binary there give an output like `dnf list <pattern>`

### Examples (using cargo run)
```bash
cargo run -- dnf5*
cargo run -- dnf5* yum* --scope installed
```

### Usage
```bash

Usage: dnf5daemon [OPTIONS] [PATTERNS]...

Arguments:
  [PATTERNS]...  packages to search for

Options:
  --scope <SCOPE>      Package scope [default: all] [possible values: all, installed, available]
  -d, --debug          Enable debug logging
  -h, --help           Print help
  -V, --version        Print version
```



## Links

- [Dnf5 dbus API](https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html)
- [zbus dbus crate](https://docs.rs/zbus/latest/zbus/)
