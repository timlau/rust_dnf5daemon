use crate::dnf::daemon::DnfDaemon;
use crate::dnf::proxy::ListResults;

use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Type, Value};

/// Macro to convert a variant store under a given key in a HashMap into a given native type
macro_rules! from_variant {
    ($pkg: expr,$typ:ty, $field:literal) => {
        match $pkg.get($field) {
            Some(v) => <$typ>::try_from(v.to_owned()).expect(concat!(
                "Can't convert ",
                $field,
                " to ",
                stringify!($typ)
            )),
            _ => panic!(concat!($field, " was not found in HashMap")),
        }
    };
}

/// Macro to put a expression into a variant (zvariant::Value)
macro_rules! to_variant {
    ($var:expr) => {
        zbus::zvariant::Value::new($var)
    }
}

/// Macro to insert a struct field in a variant type into a HashMap with same name as the field
/// insert_field! (map, self.fieldname) will add a "<fieldname>": Value(self.<fieldname>) entry
/// to the map
macro_rules! insert_field {
    ($map:expr, $self_:ident.$field:ident) => {
        $map.insert(
            stringify!($field).to_string(),
            to_variant!($self_.$field.to_owned()),
        );
    };
}

#[derive(Debug)]
pub struct DnfPackage {
    // TODO: Split the nevra up into the sub parts
    // TODO: Add more usefule fields : repo, summary
    pub name: String,
    pub arch: String,
    pub evr: String,
    pub repo_id: String,
    pub is_installed: bool,
    pub size: u64,
}

// TODO: Add setup for more useful fields, when they are added to struct
impl DnfPackage {
    pub fn from(pkg: &HashMap<String, OwnedValue>) -> DnfPackage {
        Self {
            name: from_variant!(pkg, String, "name"),
            arch: from_variant!(pkg, String, "arch"),
            evr: from_variant!(pkg, String, "evr"),
            repo_id: from_variant!(pkg, String, "repo_id"),
            is_installed: from_variant!(pkg, bool, "is_installed"),
            size: from_variant!(pkg, u64, "install_size"),
        }
    }
}

/// Stucture with options for org.rpm.dnf.v0.rpm.Rpm.list(a(sv) options.)
#[derive(Debug, Type, Deserialize, Serialize)]
struct ListOptions {
    package_attrs: Vec<String>,
    patterns: Vec<String>,
    scope: String,
    icase: bool,
    with_src: bool,
    with_nevra: bool,
    with_provides: bool,
    with_filenames: bool,
    with_binaries: bool,
}

impl ListOptions {
    fn builder() -> ListOptionsBuilder {
        ListOptionsBuilder::new()
    }

    /// Generate a HashMap with key/value (as variant) pairs to use for Dbus
    fn to_dbus(&self) -> HashMap<String, Value<'_>> {
        let mut options = HashMap::new();
        // Add a "<fieldname": Value(self.<fieldname>) entry to the map
        insert_field!(options, self.package_attrs);
        insert_field!(options, self.patterns);
        insert_field!(options, self.scope);
        insert_field!(options, self.icase);
        insert_field!(options, self.with_src);
        insert_field!(options, self.with_nevra);
        insert_field!(options, self.with_provides);
        insert_field!(options, self.with_filenames);
        insert_field!(options, self.with_binaries);
        options
    }
}

// Builder for ListOptionn
struct ListOptionsBuilder {
    package_attrs: Vec<String>,
    patterns: Vec<String>,
    scope: String,
    icase: bool,
    with_src: bool,
    with_nevra: bool,
    with_provides: bool,
    with_filenames: bool,
    with_binaries: bool,
}

impl ListOptionsBuilder {
    fn new() -> ListOptionsBuilder {
        ListOptionsBuilder {
            package_attrs: Vec::new(),
            patterns: Vec::new(),
            scope: String::from("all"),
            icase: true,
            with_src: false,
            with_nevra: true,
            with_provides: false,
            with_binaries: false,
            with_filenames: false,
        }
    }

    fn attrs(mut self, attrs: &Vec<String>) -> ListOptionsBuilder {
        for attr in attrs {
            self.package_attrs.push(attr.to_owned());
        }
        self
    }

    fn patterns(mut self, patterns: &Vec<String>) -> ListOptionsBuilder {
        for pat in patterns {
            self.patterns.push(pat.to_owned());
        }
        self
    }

    fn scope(mut self, scope: String) -> ListOptionsBuilder {
        self.scope = scope.to_owned();
        self
    }

    fn build(self) -> ListOptions {
        ListOptions {
            package_attrs: self.package_attrs,
            patterns: self.patterns,
            icase: self.icase,
            with_src: self.with_src,
            scope: self.scope,
            with_binaries: self.with_binaries,
            with_filenames: self.with_filenames,
            with_nevra: self.with_nevra,
            with_provides: self.with_provides,
        }
    }
}
/// Get packages by calling org.rpm.dnf.v0.rpm.Rpm.list()
pub async fn get_packages(
    daemon: &DnfDaemon,
    patterns: &Vec<String>,
    scope: &String,
) -> Vec<DnfPackage> {
    // Setup query options for use with org.rpm.dnf.v0.rpm.Rpm.list()
    // check here for details
    // https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html#org.rpm.dnf.v0.rpm.Rpm.list
    let attrs: Vec<String> = vec![
        "name".to_owned(),
        "install_size".to_owned(),
        "arch".to_owned(),
        "evr".to_owned(),
        "repo_id".to_owned(),
        "is_installed".to_owned(),
    ];
    let options = ListOptions::builder()
        .attrs(&attrs)
        .patterns(patterns)
        .scope(scope.to_owned())
        .build();
    debug!("{:?}", options.to_dbus());

    // Read packages from Rpm.list() and convert into DnfPackages
    let pkgs = daemon
        .rpm
        .list(options.to_dbus())
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
