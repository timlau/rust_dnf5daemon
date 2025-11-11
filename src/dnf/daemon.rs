use std::collections::HashMap;

use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::dnf;

/// Structure to store the state of the DBus connection to Dnf5daemon
pub struct DnfDaemon {
    pub session_manager: dnf::proxy::SessionManagerProxy<'static>,
    pub path: OwnedObjectPath,
    // TODO: Add the rest df the interfaces from DNF5daemon
    pub base: dnf::proxy::BaseProxy<'static>,
    pub rpm: dnf::proxy::RpmProxy<'static>,
    pub connected: bool,
}

/// methods to open/close the connection to Dnf5Daemon and setup proxies for the used interfaces
// TODO: implement the rest of the Dnf5Daemon interfaces.
impl DnfDaemon {
    pub async fn new() -> Self {
        let connection = Connection::system()
            .await
            .expect("Error: can't connect to system bus");

        let proxy = dnf::proxy::SessionManagerProxy::new(&connection)
            .await
            .expect("Error: can make dbus connection");

        let path = proxy
            .open_session(HashMap::new())
            .await
            .expect("Error: cant open dnf5daemon session");
        // let reply = proxy.open_session(HashMap::new()).await?;
        let base = dnf::proxy::BaseProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Base");
        // dbg!(&base);
        let rpm = dnf::proxy::RpmProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Rpm");

        Self {
            // connection: connection,
            session_manager: proxy,
            path: path,
            base: base,
            rpm: rpm,
            connected: true,
        }
    }

    pub async fn close(&mut self) {
        if self.connected {
            let obj_path = self.path.as_ref();
            self.session_manager
                .close_session(&obj_path)
                .await
                .expect("Error: cant close dnf5daemon session");
            self.connected = false;
        } else {
            println!("Connection is not open");
        }
    }
}
