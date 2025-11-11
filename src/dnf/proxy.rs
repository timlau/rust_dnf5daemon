use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus::proxy;
use zbus::zvariant::{OwnedValue, Type, Value};

#[derive(Debug, Type, Deserialize, Serialize)]
pub struct ListResults {
    pub items: Vec<HashMap<String, OwnedValue>>,
}

#[proxy(
    interface = "org.rpm.dnf.v0.SessionManager",
    default_service = "org.rpm.dnf.v0",
    default_path = "/org/rpm/dnf/v0"
)]
pub trait SessionManager {
    /// close_session method
    #[zbus(name = "close_session")]
    fn close_session(
        &self,
        session_object_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<bool>;

    /// open_session method
    #[zbus(name = "open_session")]
    fn open_session(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

#[proxy(interface = "org.rpm.dnf.v0.Base")]
pub trait Base {
    /// clean method
    #[zbus(name = "clean")]
    fn clean(&self, cache_type: &str) -> zbus::Result<(bool, String)>;

    /// clean_with_options method
    #[zbus(name = "clean_with_options")]
    fn clean_with_options(
        &self,
        cache_type: &str,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(bool, String)>;

    /// read_all_repos method
    #[zbus(name = "read_all_repos")]
    fn read_all_repos(&self) -> zbus::Result<bool>;

    /// reset method
    #[zbus(name = "reset")]
    fn reset(&self) -> zbus::Result<(bool, String)>;

    /// download_add_new signal
    #[zbus(signal, name = "download_add_new")]
    fn download_add_new(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        download_id: &str,
        description: &str,
        total_to_download: i64,
    ) -> zbus::Result<()>;

    /// download_end signal
    #[zbus(signal, name = "download_end")]
    fn download_end(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        download_id: &str,
        transfer_status: u32,
        message: &str,
    ) -> zbus::Result<()>;

    /// download_mirror_failure signal
    #[zbus(signal, name = "download_mirror_failure")]
    fn download_mirror_failure(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        download_id: &str,
        message: &str,
        url: &str,
        metadata: &str,
    ) -> zbus::Result<()>;

    /// download_progress signal
    #[zbus(signal, name = "download_progress")]
    fn download_progress(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        download_id: &str,
        total_to_download: i64,
        downloaded: i64,
    ) -> zbus::Result<()>;

    /// repo_key_import_request signal
    #[zbus(signal, name = "repo_key_import_request")]
    fn repo_key_import_request(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        key_id: &str,
        user_ids: Vec<&str>,
        key_fingerprint: &str,
        key_url: &str,
        timestamp: i64,
    ) -> zbus::Result<()>;
}

#[proxy(interface = "org.rpm.dnf.v0.rpm.Repo", assume_defaults = true)]
pub trait Repo {
    /// confirm_key method
    #[zbus(name = "confirm_key")]
    fn confirm_key(&self, key_id: &str, confirmed: bool) -> zbus::Result<()>;

    /// confirm_key_with_options method
    #[zbus(name = "confirm_key_with_options")]
    fn confirm_key_with_options(
        &self,
        key_id: &str,
        confirmed: bool,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// disable method
    #[zbus(name = "disable")]
    fn disable(&self, repo_ids: &[&str]) -> zbus::Result<()>;

    /// disable_with_options method
    #[zbus(name = "disable_with_options")]
    fn disable_with_options(
        &self,
        repo_ids: &[&str],
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// enable method
    #[zbus(name = "enable")]
    fn enable(&self, repo_ids: &[&str]) -> zbus::Result<()>;

    /// enable_with_options method
    #[zbus(name = "enable_with_options")]
    fn enable_with_options(
        &self,
        repo_ids: &[&str],
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// list method
    #[zbus(name = "list")]
    fn list(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[proxy(interface = "org.rpm.dnf.v0.rpm.Rpm")]
pub trait Rpm {
    /// distro_sync method
    #[zbus(name = "distro_sync")]
    fn distro_sync(
        &self,
        pkg_specs: &[&str],
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// downgrade method
    #[zbus(name = "downgrade")]
    fn downgrade(
        &self,
        pkg_specs: &[&str],
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// install method
    #[zbus(name = "install")]
    fn install(
        &self,
        pkg_specs: &[&str],
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// list method
    #[zbus(name = "list")]
    fn list(&self, options: HashMap<&str, &Value<'_>>) -> zbus::Result<ListResults>;

    /// list_fd method
    #[zbus(name = "list_fd")]
    fn list_fd(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::OwnedValue>,
        file_descriptor: zbus::zvariant::Fd<'_>,
    ) -> zbus::Result<String>;

    /// reinstall method
    #[zbus(name = "reinstall")]
    fn reinstall(
        &self,
        pkg_specs: &[&str],
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// remove method
    #[zbus(name = "remove")]
    fn remove(
        &self,
        pkg_specs: &[&str],
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// system_upgrade method
    #[zbus(name = "system_upgrade")]
    fn system_upgrade(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// upgrade method
    #[zbus(name = "upgrade")]
    fn upgrade(
        &self,
        pkg_specs: &[&str],
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// transaction_action_progress signal
    #[zbus(signal, name = "transaction_action_progress")]
    fn transaction_action_progress(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        nevra: &str,
        processed: u64,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_action_start signal
    #[zbus(signal, name = "transaction_action_start")]
    fn transaction_action_start(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        nevra: &str,
        action: u32,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_action_stop signal
    #[zbus(signal, name = "transaction_action_stop")]
    fn transaction_action_stop(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        nevra: &str,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_after_complete signal
    #[zbus(signal, name = "transaction_after_complete")]
    fn transaction_after_complete(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        success: bool,
    ) -> zbus::Result<()>;

    /// transaction_before_begin signal
    #[zbus(signal, name = "transaction_before_begin")]
    fn transaction_before_begin(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_elem_progress signal
    #[zbus(signal, name = "transaction_elem_progress")]
    fn transaction_elem_progress(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        nevra: &str,
        processed: u64,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_script_error signal
    #[zbus(signal, name = "transaction_script_error")]
    fn transaction_script_error(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        nevra: &str,
        scriptlet_type: u32,
        return_code: u64,
    ) -> zbus::Result<()>;

    /// transaction_script_start signal
    #[zbus(signal, name = "transaction_script_start")]
    fn transaction_script_start(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        nevra: &str,
        scriptlet_type: u32,
    ) -> zbus::Result<()>;

    /// transaction_script_stop signal
    #[zbus(signal, name = "transaction_script_stop")]
    fn transaction_script_stop(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        nevra: &str,
        scriptlet_type: u32,
        return_code: u64,
    ) -> zbus::Result<()>;

    /// transaction_transaction_progress signal
    #[zbus(signal, name = "transaction_transaction_progress")]
    fn transaction_transaction_progress(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        processed: u64,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_transaction_start signal
    #[zbus(signal, name = "transaction_transaction_start")]
    fn transaction_transaction_start(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_transaction_stop signal
    #[zbus(signal, name = "transaction_transaction_stop")]
    fn transaction_transaction_stop(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_unpack_error signal
    #[zbus(signal, name = "transaction_unpack_error")]
    fn transaction_unpack_error(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        nevra: &str,
    ) -> zbus::Result<()>;

    /// transaction_verify_progress signal
    #[zbus(signal, name = "transaction_verify_progress")]
    fn transaction_verify_progress(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        processed: u64,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_verify_start signal
    #[zbus(signal, name = "transaction_verify_start")]
    fn transaction_verify_start(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        total: u64,
    ) -> zbus::Result<()>;

    /// transaction_verify_stop signal
    #[zbus(signal, name = "transaction_verify_stop")]
    fn transaction_verify_stop(
        &self,
        session_object_path: zbus::zvariant::ObjectPath<'_>,
        total: u64,
    ) -> zbus::Result<()>;
}

#[proxy(interface = "org.rpm.dnf.v0.Goal", assume_defaults = true)]
pub trait Goal {
    /// cancel method
    #[zbus(name = "cancel")]
    fn cancel(&self) -> zbus::Result<(bool, String)>;

    /// do_transaction method
    #[zbus(name = "do_transaction")]
    fn do_transaction(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// get_transaction_problems method
    #[zbus(name = "get_transaction_problems")]
    fn get_transaction_problems(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// get_transaction_problems_string method
    #[zbus(name = "get_transaction_problems_string")]
    fn get_transaction_problems_string(&self) -> zbus::Result<Vec<String>>;

    /// reset method
    #[zbus(name = "reset")]
    fn reset(&self) -> zbus::Result<()>;

    /// resolve method
    #[zbus(name = "resolve")]
    #[allow(clippy::type_complexity)]
    fn resolve(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(
        Vec<(
            String,
            String,
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
        u32,
    )>;
}

#[proxy(interface = "org.rpm.dnf.v0.Offline", assume_defaults = true)]
pub trait Offline {
    /// cancel method
    #[zbus(name = "cancel")]
    fn cancel(&self) -> zbus::Result<(bool, String)>;

    /// cancel_with_options method
    #[zbus(name = "cancel_with_options")]
    fn cancel_with_options(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(bool, String)>;

    /// clean method
    #[zbus(name = "clean")]
    fn clean(&self) -> zbus::Result<(bool, String)>;

    /// clean_with_options method
    #[zbus(name = "clean_with_options")]
    fn clean_with_options(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(bool, String)>;

    /// get_status method
    #[zbus(name = "get_status")]
    fn get_status(
        &self,
    ) -> zbus::Result<(
        bool,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;

    /// set_finish_action method
    #[zbus(name = "set_finish_action")]
    fn set_finish_action(&self, action: &str) -> zbus::Result<(bool, String)>;

    /// set_finish_action_with_options method
    #[zbus(name = "set_finish_action_with_options")]
    fn set_finish_action_with_options(
        &self,
        action: &str,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(bool, String)>;
}

#[proxy(interface = "org.rpm.dnf.v0.comps.Group", assume_defaults = true)]
pub trait Group {
    /// list method
    #[zbus(name = "list")]
    fn list(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}

#[proxy(interface = "org.rpm.dnf.v0.History", assume_defaults = true)]
pub trait History {
    /// recent_changes method
    #[zbus(name = "recent_changes")]
    fn recent_changes(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<
        std::collections::HashMap<
            String,
            Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>,
        >,
    >;
}

#[proxy(interface = "org.rpm.dnf.v0.Advisory", assume_defaults = true)]
pub trait Advisory {
    /// list method
    #[zbus(name = "list")]
    fn list(
        &self,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
}
