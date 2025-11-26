# Rust crate to demo use of dnf5 dbus API-

The code and is under heavy development and no way complete
But can be used for inspiration on how to use the dnf5 dbus API

(C) 2025 Tim Lauridsen

License: MIT

## How to test

I have created a simple demo application located here:

https://github.com/timlau/minidnf


## Examples

### Simple example to the high-level get_packages API to list packages matching a given pattern.
```rust
cargo run --example e01_list
```

## Links

- [Dnf5 dbus API](https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html)
- [zbus dbus crate](https://docs.rs/zbus/latest/zbus/)
