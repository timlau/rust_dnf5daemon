mod dnf;

use std::collections::HashMap;

use std::error::Error;

use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue, Value},
};

use crate::dnf::ListResults;

const RPM_LIST_ATTR: &[&str] = &["nevra", "install_size"];

#[derive(Debug)]
pub struct DnfPackage {
    nevra: String,
    size: u64,
}

impl DnfPackage {
    fn from(pkg: &HashMap<String, OwnedValue>) -> DnfPackage {
        let nevra = String::try_from(pkg.get("nevra").unwrap().to_owned());
        let size = u64::try_from(pkg.get("install_size").unwrap().to_owned());
        Self {
            nevra: nevra.unwrap(),
            size: size.unwrap(),
        }
    }
}

async fn get_packages(daemon: &DnfDaemon, patterns: &[&str]) -> Vec<DnfPackage> {
    // Setup Options
    let mut options = HashMap::new();
    let package_attr = Value::new(RPM_LIST_ATTR.to_vec());
    let patterns = Value::new(patterns.to_vec());
    options.insert("package_attrs", &package_attr);
    options.insert("patterns", &patterns);

    // Read packages from Rpm.list()
    let pkgs = daemon
        .rpm
        .list(options)
        .await
        .expect("org.rpm.dnf.v0.Rpm.list failed");
    build_packages(&pkgs)
}

fn build_packages(pkgs: &ListResults) -> Vec<DnfPackage> {
    let mut packages = Vec::new();
    for pkg in &pkgs.items {
        packages.push(DnfPackage::from(pkg));
    }
    packages
}

struct DnfDaemon {
    // connection: Connection,
    session_manager: dnf::SessionManagerProxy<'static>,
    path: OwnedObjectPath,
    base: dnf::BaseProxy<'static>,
    rpm: dnf::RpmProxy<'static>,
    connected: bool,
}

impl DnfDaemon {
    async fn new() -> Self {
        let connection = Connection::system()
            .await
            .expect("Error: can't connect to system bus");

        let proxy = dnf::SessionManagerProxy::new(&connection)
            .await
            .expect("Error: can make dbus connection");

        let path = proxy
            .open_session(HashMap::new())
            .await
            .expect("Error: cant open dnf5daemon session");
        // let reply = proxy.open_session(HashMap::new()).await?;
        let base = dnf::BaseProxy::builder(&connection)
            .path(path.clone())
            .unwrap()
            .destination("org.rpm.dnf.v0")
            .unwrap()
            .build()
            .await
            .expect("Error: cant connect to org.rpm.dnf.v0.Base");
        // dbg!(&base);
        let rpm = dnf::RpmProxy::builder(&connection)
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

    async fn close(&mut self) {
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

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut dnf_daemon = DnfDaemon::new().await;
    dnf_daemon.base.read_all_repos().await.ok();
    let packages = get_packages(&dnf_daemon, &["yum*"]).await;
    for pkg in packages {
        println!("{} - {}", pkg.nevra, pkg.size);
    }
    dnf_daemon.close().await;

    Ok(())
}
