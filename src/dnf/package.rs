use crate::dnf;
use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Value};

const RPM_LIST_ATTR: &[&str] = &["nevra", "install_size"];

#[derive(Debug)]
pub struct DnfPackage {
    pub nevra: String,
    pub size: u64,
}

impl DnfPackage {
    pub fn from(pkg: &HashMap<String, OwnedValue>) -> DnfPackage {
        let nevra = String::try_from(pkg.get("nevra").unwrap().to_owned());
        let size = u64::try_from(pkg.get("install_size").unwrap().to_owned());
        Self {
            nevra: nevra.unwrap(),
            size: size.unwrap(),
        }
    }
}

pub async fn get_packages(daemon: &dnf::daemon::DnfDaemon, patterns: &[&str]) -> Vec<DnfPackage> {
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

pub fn build_packages(pkgs: &dnf::proxy::ListResults) -> Vec<DnfPackage> {
    let mut packages = Vec::new();
    for pkg in &pkgs.items {
        packages.push(DnfPackage::from(pkg));
    }
    packages
}
