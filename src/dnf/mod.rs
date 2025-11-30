/// This module contains a struct and methods to handle the connection to dnf5daemon-server.
pub mod daemon;

/// This module contain functions and struct the is a more high level abstation to
/// using the low-levet DNF5 DBus API
pub mod package;

/// This module contains Traits for the DBus interfaces that maps the Dbus API of dnf5daemon-server.
pub(crate) mod proxy;

/// This module contains sruct for handling DNF Transactions via the Dbus API.
pub mod transaction;
