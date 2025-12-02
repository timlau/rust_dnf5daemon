use crate::dnf::daemon::DnfDaemon;
use crate::dnf::proxy::ListResults;

use derive_more::From;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::zvariant::{Error, OwnedValue, Type, Value};

// region:    --- macroes

/// Macro to convert a variant store under a given key in a HashMap into a given native type
macro_rules! from_variant {
    ($pkg: expr,$typ:ty, $field:literal) => {
        match $pkg.get($field) {
            Some(v) => <$typ>::try_from(v.to_owned()),
            None => Err(Error::Message(format!(
                "Key {:?} not found in Rpm.list() result.",
                $field
            ))),
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
        $map.insert(stringify!($field).to_string(), to_variant!($self_.$field.to_owned()));
    };
}

// endregion: --- macroes

// region:    --- PackageAttr

#[derive(Debug, From, Serialize, Deserialize, Type)]
pub enum PackageAttr {
    Name,
    Epoch,
    Version,
    Release,
    Arch,
    RepoId,
    FromRepoId,
    IsInstalled,
    InstallSize,
    DownloadSize,
    Buildtime,
    Sourcerpm,
    Summary,
    Url,
    License,
    Description,
    Files,
    Changelogs,
    Provides,
    Requires,
    RequiresPre,
    Conflicts,
    Obsoletes,
    Recommends,
    Suggests,
    Enhances,
    Supplements,
    Evr,
    Nevra,
    FullNevra,
    Reason,
    Vendor,
    Group,
}

impl From<String> for PackageAttr {
    fn from(attr: String) -> Self {
        match attr.to_lowercase().as_str() {
            "name" => PackageAttr::Name,
            "epoch" => PackageAttr::Epoch,
            "version" => PackageAttr::Version,
            "release" => PackageAttr::Release,
            "arch" => PackageAttr::Arch,
            "repo_id" => PackageAttr::RepoId,
            "from_repo_id" => PackageAttr::FromRepoId,
            "is_installed" => PackageAttr::IsInstalled,
            "install_size" => PackageAttr::InstallSize,
            "download_size" => PackageAttr::DownloadSize,
            "buildtime" => PackageAttr::Buildtime,
            "sourcerpm" => PackageAttr::Sourcerpm,
            "summary" => PackageAttr::Summary,
            "url" => PackageAttr::Url,
            "license" => PackageAttr::License,
            "description" => PackageAttr::Description,
            "files" => PackageAttr::Files,
            "changelogs" => PackageAttr::Changelogs,
            "provides" => PackageAttr::Provides,
            "requires" => PackageAttr::Requires,
            "requires_pre" => PackageAttr::RequiresPre,
            "conflicts" => PackageAttr::Conflicts,
            "obsoletes" => PackageAttr::Obsoletes,
            "recommends" => PackageAttr::Recommends,
            "suggests" => PackageAttr::Suggests,
            "enhances" => PackageAttr::Enhances,
            "supplements" => PackageAttr::Supplements,
            "evr" => PackageAttr::Evr,
            "nevra" => PackageAttr::Nevra,
            "full_nevra" => PackageAttr::FullNevra,
            "reason" => PackageAttr::Reason,
            "vendor" => PackageAttr::Vendor,
            "group" => PackageAttr::Group,
            _ => PackageAttr::Name, // default to name
        }
    }
}

impl core::fmt::Display for PackageAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PackageAttr::Name => "name",
            PackageAttr::Epoch => "epoch",
            PackageAttr::Version => "version",
            PackageAttr::Release => "release",
            PackageAttr::Arch => "arch",
            PackageAttr::RepoId => "repo_id",
            PackageAttr::FromRepoId => "from_repo_id",
            PackageAttr::IsInstalled => "is_installed",
            PackageAttr::InstallSize => "install_size",
            PackageAttr::DownloadSize => "download_size",
            PackageAttr::Buildtime => "buildtime",
            PackageAttr::Sourcerpm => "sourcerpm",
            PackageAttr::Summary => "summary",
            PackageAttr::Url => "url",
            PackageAttr::License => "license",
            PackageAttr::Description => "description",
            PackageAttr::Files => "files",
            PackageAttr::Changelogs => "changelogs",
            PackageAttr::Provides => "provides",
            PackageAttr::Requires => "requires",
            PackageAttr::RequiresPre => "requires_pre",
            PackageAttr::Conflicts => "conflicts",
            PackageAttr::Obsoletes => "obsoletes",
            PackageAttr::Recommends => "recommends",
            PackageAttr::Suggests => "suggests",
            PackageAttr::Enhances => "enhances",
            PackageAttr::Supplements => "supplements",
            PackageAttr::Evr => "evr",
            PackageAttr::Nevra => "nevra",
            PackageAttr::FullNevra => "full_nevra",
            PackageAttr::Reason => "reason",
            PackageAttr::Vendor => "vendor",
            PackageAttr::Group => "group",
        };
        write!(f, "{s}")
    }
}

// endregion: --- PackageAttr

// region:    --- Scope

#[derive(Debug, From, Serialize, Deserialize, Type)]
pub enum Scope {
    All,
    Installed,
    Available,
    Upgrades,
    Upgradable,
}

impl core::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Scope::All => "all",
            Scope::Installed => "installed",
            Scope::Available => "available",
            Scope::Upgrades => "upgrades",
            Scope::Upgradable => "upgradable",
        };
        write!(f, "{s}")
    }
}

impl From<&str> for Scope {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "all" => Scope::All,
            "installed" => Scope::Installed,
            "available" => Scope::Available,
            "upgrades" => Scope::Upgrades,
            "upgradable" => Scope::Upgradable,
            _ => Scope::All, // default to all
        }
    }
}

// endregion: --- Scope

// region:    --- DnfPackage
/// a native rust struct to represent a dnf package
/// it is designed to be used with get_packages
#[derive(Debug)]
pub struct DnfPackage {
    pub name: String,
    pub arch: String,
    pub evr: String,
    pub repo_id: String,
    pub is_installed: bool,
    pub size: u64,
}

impl AsRef<DnfPackage> for DnfPackage {
    fn as_ref(&self) -> &DnfPackage {
        self
    }
}

// TODO: Add setup for more useful fields, when they are added to struct
impl DnfPackage {
    /// build a native DnfPackage from a HashMap contains values returned from call to Rpm.list() method
    /// with the following attrs: `name, arch, evr, repo_id, is_installed, installed_size`
    /// it is designed to be used with get_packages
    pub fn from(pkg: &HashMap<String, OwnedValue>) -> Result<DnfPackage, Error> {
        Ok(Self {
            name: from_variant!(pkg, String, "name")?,
            arch: from_variant!(pkg, String, "arch")?,
            evr: from_variant!(pkg, String, "evr")?,
            repo_id: from_variant!(pkg, String, "repo_id")?,
            is_installed: from_variant!(pkg, bool, "is_installed")?,
            size: from_variant!(pkg, u64, "install_size")?,
        })
    }
}

// endregion: --- DnfPackage

// region:    --- ListOptions

/// Stucture with options for org.rpm.dnf.v0.rpm.Rpm.list(a(sv) options.)
#[derive(Debug, Type, Deserialize, Serialize)]
pub struct ListOptions {
    package_attrs: Vec<PackageAttr>,
    patterns: Vec<String>,
    scope: Scope,
    icase: bool,
    with_src: bool,
    with_nevra: bool,
    with_provides: bool,
    with_filenames: bool,
    with_binaries: bool,
}

impl AsRef<ListOptions> for ListOptions {
    fn as_ref(&self) -> &ListOptions {
        self
    }
}

impl ListOptions {
    /// create a ListOptionBuilder to build the wanted options
    pub fn builder() -> ListOptionsBuilder {
        ListOptionsBuilder::new()
    }

    /// Generate a HashMap with key/value (as variant) pairs to use for Dbus
    pub fn to_dbus(&self) -> HashMap<String, Value<'_>> {
        let mut options = HashMap::new();
        // Add a "<fieldname": Value(self.<fieldname>) entry to the map
        let pa = Value::new(self.package_attrs.iter().map(|attr| attr.to_string()).collect::<Vec<String>>());
        let scope = Value::new(self.scope.to_string());
        options.insert("package_attrs".to_string(), pa);
        options.insert("scope".to_string(), scope);
        insert_field!(options, self.patterns);
        insert_field!(options, self.icase);
        insert_field!(options, self.with_src);
        insert_field!(options, self.with_nevra);
        insert_field!(options, self.with_provides);
        insert_field!(options, self.with_filenames);
        insert_field!(options, self.with_binaries);
        options
    }
}

// -- TODO: scope should be an enum (“all”, “installed”, “available”, “upgrades”, “upgradable”)
// -- TODO: package_attrs should be an enum ( name, epoch, version, release, arch, repo_id, from_repo_id, is_installed, install_size, download_size,
// --  buildtime, sourcerpm, summary, url, license, description, files, changelogs, provides, requires, requires_pre, conflicts, obsoletes, recommends,
// --  suggests, enhances, supplements, evr, nevra, full_nevra, reason, vendor, group.)
/// Builder for setup ListOptions
pub struct ListOptionsBuilder {
    package_attrs: Vec<PackageAttr>,
    patterns: Vec<String>,
    scope: Scope,
    icase: bool,
    with_src: bool,
    with_nevra: bool,
    with_provides: bool,
    with_filenames: bool,
    with_binaries: bool,
}

impl Default for ListOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ListOptionsBuilder {
    /// make a new ListOptionBuilder object.
    pub fn new() -> ListOptionsBuilder {
        ListOptionsBuilder {
            package_attrs: Vec::new(),
            patterns: Vec::new(),
            scope: Scope::All,
            icase: true,
            with_src: false,
            with_nevra: true,
            with_provides: false,
            with_binaries: false,
            with_filenames: false,
        }
    }

    /// Add attributes to return from the matched packages.
    pub fn attrs(mut self, attrs: Vec<PackageAttr>) -> ListOptionsBuilder {
        for attr in attrs {
            self.package_attrs.push(attr);
        }
        self
    }

    /// Add patterns to match
    pub fn patterns(mut self, patterns: &Vec<String>) -> ListOptionsBuilder {
        for pat in patterns {
            self.patterns.push(pat.to_owned());
        }
        self
    }

    /// Add scope to search in (all, installed, available)
    pub fn scope(mut self, scope: Scope) -> ListOptionsBuilder {
        self.scope = scope;
        self
    }

    /// build the ListOption object from the applied options
    pub fn build(self) -> ListOptions {
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

// endregion: --- ListOptions

/// Get packages by calling org.rpm.dnf.v0.rpm.Rpm.list()
pub async fn get_packages(
    daemon: impl AsRef<DnfDaemon>,
    patterns: impl AsRef<Vec<String>>,
    scope: Scope,
) -> Result<Vec<DnfPackage>, Error> {
    // Setup query options for use with org.rpm.dnf.v0.rpm.Rpm.list()
    // check here for details
    // https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html#org.rpm.dnf.v0.rpm.Rpm.list
    let attrs: Vec<PackageAttr> = vec![
        PackageAttr::Name,
        PackageAttr::InstallSize,
        PackageAttr::Arch,
        PackageAttr::Evr,
        PackageAttr::RepoId,
        PackageAttr::IsInstalled,
    ];
    let options = ListOptions::builder()
        .attrs(attrs)
        .patterns(patterns.as_ref())
        .scope(scope)
        .build();
    // println!("{:?}", options.to_dbus());

    // Read packages from Rpm.list() and convert into DnfPackages
    let pkgs = daemon
        .as_ref()
        .rpm
        .list(options.to_dbus())
        .await
        .expect("org.rpm.dnf.v0.Rpm.list failed");
    // println!("Raw packages: {:?}", pkgs);
    build_packages(&pkgs)
}

/// Convert the package HashMap's returnend by zbus to DnfPackage objects
fn build_packages(pkgs: &ListResults) -> Result<Vec<DnfPackage>, Error> {
    let mut packages = Vec::new();
    for pkg in &pkgs.items {
        packages.push(DnfPackage::from(pkg)?);
    }
    Ok(packages)
}
