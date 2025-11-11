use crate::dnf::daemon::DnfDaemon;
use crate::dnf::proxy::ListResults;
use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Value};

const RPM_LIST_ATTR: &[&str] = &["nevra", "install_size"];

#[derive(Debug)]
pub struct DnfPackage {
    // TODO: Split the nevra up into the sub parts
    // TODO: Add more usefule fields : repo, summary
    pub nevra: String,
    pub size: u64,
}

// TODO: Add setup for more useful fields, when they are added to struct
impl DnfPackage {
    pub fn from(pkg: &HashMap<String, OwnedValue>) -> DnfPackage {
        // Convert the OwnedValue<Value<Str>> into a native String
        let nevra = String::try_from(pkg.get("nevra").unwrap().to_owned());
        // Convert the OwnedValue<Value<U64>> into a native U64
        let size = u64::try_from(pkg.get("install_size").unwrap().to_owned());
        Self {
            nevra: nevra.unwrap(),
            size: size.unwrap(),
        }
    }
}

/// Get packages by calling org.rpm.dnf.v0.rpm.Rpm.list()
pub async fn get_packages(daemon: &DnfDaemon, patterns: &[&str]) -> Vec<DnfPackage> {
    // Setup query options for use with org.rpm.dnf.v0.rpm.Rpm.list()
    // check here for details
    // https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html#org.rpm.dnf.v0.rpm.Rpm.list
    let mut options = HashMap::new();
    let package_attr = Value::new(RPM_LIST_ATTR.to_vec()); // package atributes to return from query
    let patterns = Value::new(patterns.to_vec()); // wildcart to query for
    let icase = Value::new(true); // ignore case in query
    let with_src = Value::new(false); // don't return source packages
    options.insert("package_attrs", &package_attr);
    options.insert("patterns", &patterns);
    options.insert("with_src", &with_src);
    options.insert("icase", &icase);

    // Read packages from Rpm.list() and convert into DnfPackages
    let pkgs = daemon
        .rpm
        .list(options)
        .await
        .expect("org.rpm.dnf.v0.Rpm.list failed");
    build_packages(&pkgs)
}

/// Convert the package HashMap's returnend by zbus to DnfPackage objects
pub fn build_packages(pkgs: &ListResults) -> Vec<DnfPackage> {
    let mut packages = Vec::new();
    for pkg in &pkgs.items {
        packages.push(DnfPackage::from(pkg));
    }
    packages
}
