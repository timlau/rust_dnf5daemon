# Rust module to demo use of dnf5 dbus API from Rust.

This not ready to use code and is under heavy development.

But can be used for inspiration.

(C) 2025 Tim Lauridsen

License: MIT

## How to run

To get a list of packages & sizes for a pattern

```bash
RUST_LOG=debug cargo run -- dnf5*
```

## Links

- [Dnf5 dbus API](https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html)
- [zbus dbus crate](https://docs.rs/zbus/latest/zbus/)
