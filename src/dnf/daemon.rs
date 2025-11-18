#![allow(unused_imports)]
use log::{debug, info, trace, warn};
use std::collections::HashMap;

use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::dnf;

/// This does all the work by creating a new session to the dnf5daemon-server.
/// Store proxies to the Dbus interfaces publised be the dnf5daemon-server.
/// Automatic close the session, when the instance is dropped.
/// So no session will be kept running on the dnf5daemon is the user application panics.

#[derive(Debug)]
pub struct DnfDaemon {
    /// proxy for interface org.rpm.dnf.v0.SessionManger
    pub session_manager: dnf::proxy::SessionManagerProxy<'static>,
    pub path: OwnedObjectPath,
    /// proxy for interface org.rpm.dnf.v0.Base
    pub base: dnf::proxy::BaseProxy<'static>,
    /// proxy for interface org.rpm.dnf.v0.Rpm
    pub rpm: dnf::proxy::RpmProxy<'static>,
    /// proxy for interface org.rpm.dnf.v0.Repo
    pub repo: dnf::proxy::RepoProxy<'static>,
    /// proxy for interface org.rpm.dnf.v0.Goal
    pub goal: dnf::proxy::GoalProxy<'static>,
    /// proxy for interface org.rpm.dnf.v0.Group
    pub group: dnf::proxy::GroupProxy<'static>,
    /// proxy for interface org.rpm.dnf.v0.Offline
    pub offline: dnf::proxy::OfflineProxy<'static>,
    /// proxy for interface org.rpm.dnf.v0.Advisory
    pub advisory: dnf::proxy::AdvisoryProxy<'static>,
    /// session connect status
    connected: bool,
}

impl AsRef<DnfDaemon> for DnfDaemon {
    fn as_ref(&self) -> &DnfDaemon {
        self
    }
}
/// methods to open/close the connection to dnf5daemon-server and setup proxies for the used interfaces
impl DnfDaemon {
    pub async fn new() -> Result<DnfDaemon, zbus::Error> {
        let connection = Connection::system().await?;

        // proxy for interface org.rpm.dnf.v0.SessionManger
        let proxy = dnf::proxy::SessionManagerProxy::new(&connection).await?;

        let path = proxy.open_session(HashMap::new()).await?;

        // proxy for interface org.rpm.dnf.v0.Base
        let base = dnf::proxy::BaseProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await?;

        // proxy for interface org.rpm.dnf.v0.Rpm
        let rpm = dnf::proxy::RpmProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await?;

        // proxy for interface org.rpm.dnf.v0.Repo
        let repo = dnf::proxy::RepoProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await?;

        // proxy for interface org.rpm.dnf.v0.Goal
        let goal = dnf::proxy::GoalProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await?;

        // proxy for interface org.rpm.dnf.v0.Group
        let group = dnf::proxy::GroupProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await?;

        // proxy for interface org.rpm.dnf.v0.Advisory
        let advisory = dnf::proxy::AdvisoryProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await?;

        // proxy for interface org.rpm.dnf.v0.Offline
        let offline = dnf::proxy::OfflineProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await?;
        debug!("DBUS: org.rpm.dnf.v0 session opened : {path}");
        Ok(Self {
            session_manager: proxy,
            path: path,
            base: base,
            rpm: rpm,
            repo: repo,
            goal: goal,
            group: group,
            offline: offline,
            advisory: advisory,
            connected: true,
        })
    }

    /// close the session to dnf5daemon-server, it is called automatic when the object is dropped.
    pub async fn close(&mut self) -> Result<bool, &str> {
        if self.connected {
            let obj_path = self.path.as_ref();
            self.session_manager
                .close_session(&obj_path)
                .await
                .expect("Error: cant close org.rpm.dnf.v0 session");
            self.connected = false;
            return Ok(self.connected.clone());
        } else {
            warn!("org.rpm.dnf.v0 session is not open");
            return Err("org.rpm.dnf.v0 session is not open");
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected.to_owned()
    }
}

impl Drop for DnfDaemon {
    /// make sure that any existing session with dnf5daemon-server is closed
    fn drop(&mut self) {
        if self.connected {
            let path = self.path.to_owned().to_string();
            match futures::executor::block_on(self.close()) {
                Ok(_) => debug!("DBUS: org.rpm.dnf.v0 session closed : {}", path),
                Err(e) => warn!("org.rpm.dnf.v0 session close error : {}", e),
            }
        }
    }
}
