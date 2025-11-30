//! Library to connect to the [dnf5 Dbus API](https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html) (dnf5daemon-server)
//!
//! It contain the main functionality to:
//! - Open session to the dnf5daemon-server .
//! - Setup proxies to the different interfaces published by the dnf5daemon-server.
//! - Automatic closing of the session when object is `DnfDaemon` instance is droppd
//! - there is also a get-package method to use the `org.rpm.dnf.V0.rpm.list` method to get packages matching given options.
//!
//! ## Example
//! A simple example, that
//!
//! - Make a new session to the dnf5daemon-server.
//! - Send a command to make the dnf5daemon load repo metadata
//! - Request packages that match a pattern
//! - List the local `DnfPackage` instances that represent the result of the request
//!
//! ``` rust
//! use std::error::Error;
//! use dnf5daemon::package::get_packages;
//! use dnf5daemon::DnfDaemon;
//! use dnf5daemon::package::DnfPackage;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     if let Ok(dnf_daemon) = DnfDaemon::new().await {
//!         let rc = dnf_daemon.base.read_all_repos().await.ok().unwrap();
//!         let pattern: Vec<String> = vec!["dnf5*".to_owned()];
//!         let packages = &get_packages(&dnf_daemon, &pattern, &"all".to_owned())
//!             .await
//!             .expect("Error in get_packages");
//!         for pkg in packages {
//!            println!("{:?}", pkg);
//!         }
//!     } else {
//!         println!("Could not connect to dnf5daemon-server");
//!     };
//!     Ok(())
//! }
//! ```

/// the root module of the library
mod dnf;
mod errors;

// re-exports
pub use crate::dnf::daemon::DnfDaemon;
pub use crate::dnf::package;
pub use crate::dnf::transaction;
pub use crate::errors::{Error, Result};
