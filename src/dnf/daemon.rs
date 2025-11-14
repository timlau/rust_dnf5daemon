#![allow(unused_imports)]
use log::{debug, info, trace, warn};
use std::collections::HashMap;

use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::dnf;

/// Structure to store the state of the DBus connection to Dnf5daemon
pub struct DnfDaemon {
    pub session_manager: dnf::proxy::SessionManagerProxy<'static>,
    pub path: OwnedObjectPath,
    pub base: dnf::proxy::BaseProxy<'static>,
    pub rpm: dnf::proxy::RpmProxy<'static>,
    pub repo: dnf::proxy::RepoProxy<'static>,
    pub group: dnf::proxy::GroupProxy<'static>,
    pub offline: dnf::proxy::OfflineProxy<'static>,
    pub advisory: dnf::proxy::AdvisoryProxy<'static>,
    pub connected: bool,
}

/// methods to open/close the connection to Dnf5Daemon and setup proxies for the used interfaces
impl DnfDaemon {
    pub async fn new() -> Self {
        let connection = Connection::system()
            .await
            .expect("Error: can't connect to system bus");

        // proxy for interface org.rpm.dnf.v0.SessionManger
        let proxy = dnf::proxy::SessionManagerProxy::new(&connection)
            .await
            .expect("Error: can make dbus connection");

        let path = proxy
            .open_session(HashMap::new())
            .await
            .expect("Error: cant open dnf5daemon session");

        // proxy for interface org.rpm.dnf.v0.Base
        let base = dnf::proxy::BaseProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Base");

        // proxy for interface org.rpm.dnf.v0.Rpm
        let rpm = dnf::proxy::RpmProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Rpm");

        // proxy for interface org.rpm.dnf.v0.Repo
        let repo = dnf::proxy::RepoProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Repo");

        // proxy for interface org.rpm.dnf.v0.Group
        let group = dnf::proxy::GroupProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Group");

        // proxy for interface org.rpm.dnf.v0.Advisory
        let advisory = dnf::proxy::AdvisoryProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Advisory");

        // proxy for interface org.rpm.dnf.v0.Offline
        let offline = dnf::proxy::OfflineProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Offline");
        debug!("dnf5daemon session opened : {path}");
        Self {
            session_manager: proxy,
            path: path,
            base: base,
            rpm: rpm,
            repo: repo,
            group: group,
            offline: offline,
            advisory: advisory,
            connected: true,
        }
    }

    pub async fn close(&mut self) -> Result<bool, &str> {
        if self.connected {
            let obj_path = self.path.as_ref();
            self.session_manager
                .close_session(&obj_path)
                .await
                .expect("Error: cant close dnf5daemon session");
            self.connected = false;
            return Ok(self.connected.clone());
        } else {
            warn!("Connection is not open");
            return Err("Connection is not open");
        }
    }
}

impl Drop for DnfDaemon {
    fn drop(&mut self) {
        if self.connected {
            match futures::executor::block_on(self.close()) {
                Ok(_) => debug!("Dnfdaemon closed"),
                Err(e) => warn!("Dnfdaemon close error : {}", e),
            }
        }
    }
}
