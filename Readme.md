# Rust libraty to use the dnf5 dbus API-

The code and is under heavy development and no way complete or API is **NOT** stable.
But can be used for inspiration on how to use the dnf5 dbus API

(C) 2025 Tim Lauridsen

License: MIT

## How to test

I have created a simple demo application located here:

https://github.com/timlau/minidnf


## Examples

### Example to the high-level get_packages API to list packages matching a given pattern.
```rust
cargo run --example list
```

### Example to use the transaction API to install the 0xFFFF package
```rust
cargo run --example install
```

### Example to use the transaction API to remove the 0xFFFF package
```rust
cargo run --example remove
```

## Links

- [Dnf5 dbus API](https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html)
- [zbus dbus crate](https://docs.rs/zbus/latest/zbus/)
