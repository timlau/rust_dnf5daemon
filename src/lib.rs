//! Library to connect to the [dnf5 Dbus API](https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html) (dnf5daemon-server)
//!
//! It contain the main functionality to:
//! - Open session to the dnf5daemon-server .
//! - Setup proxies to the different interfaces published by the dnf5daemon-server.
//! - Automatic closing of the session when object is `DnfDaemon` instance is droppd
//! - there is also a get-package method to use the `org.rpm.dnf.V0.rpm.list` method to get packages matching given options.
//! - There is also a `Transaction` struct to handle transactions via the dnf5daemon Dbus API.
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
//! use dnf5daemon::package::{Scope, get_packages};
//! use dnf5daemon::{DnfDaemon, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Try to connect to the dnf5daemon server using the default configuration
//!     match DnfDaemon::default().await {
//!         Ok(dnf_daemon) => {
//!             // Ensure repositories are loaded (matches other examples' pattern)
//!             match dnf_daemon.base.read_all_repos().await {
//!                 Ok(rc) => println!("Read all repos returned: {:?}", rc),
//!                 Err(e) => eprintln!("Warning: read_all_repos failed: {:?}", e),
//!             }
//!
//!             // Use a simple pattern and list matching packages
//!             let pattern: Vec<String> = vec![String::from("dnf5*")];
//!             println!("Searching for packages matching: {:?}", pattern);
//!
//!             match get_packages(&dnf_daemon, pattern, Scope::All).await {
//!                 Ok(pkgs) => {
//!                     println!("Found {} packages:", pkgs.len());
//!                     for pkg in pkgs.iter().take(50) {
//!                         println!("  {:?}", pkg);
//!                     }
//!                     if pkgs.len() > 50 {
//!                         println!("  ... ({} more results)", pkgs.len() - 50);
//!                     }
//!                 }
//!                 Err(e) => {
//!                     eprintln!("Error while getting packages: {:?}", e);
//!                 }
//!             }
//!         }
//!         Err(_) => {
//!             eprintln!("Could not connect to dnf5daemon-server");
//!         }
//!     }
//!
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
