#[allow(unused)]
#[allow(dead_code)]
use crate::DnfDaemon;
use crate::{Error, Result};
use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Value};

// region:    --- Types

// -- Custom type for Options Hashmap used in various method calls
type Options = HashMap<&'static str, &'static Value<'static>>;

// -- Custom type for TransactionPackage
type TransactionPackage = HashMap<String, OwnedValue>;

// -- Custom type representing one member of the transaction returned by Goal.resolve()
// -- it contains a typle of (id, action, reason, item attributes, tx_pkg)
// -- See https://dnf5.readthedocs.io/en/latest/dnf_daemon/dnf5daemon_dbus_api.8.html#org.rpm.dnf.v0.Goal
type TransactionItem = (String, String, String, HashMap<String, OwnedValue>, TransactionPackage);

// endregion: --- Types

// region:    --- Enums

/// Enum representing a transaction action to be performed on a package
// -- check dnf5 source code:
// -- https://github.com/rpm-software-management/dnf5/blob/3739c4a34db6e7abcd8b4faf0db7d5307f37d340/dnf5daemon-server/transaction.cpp#L30
// -- https://github.com/rpm-software-management/dnf5/blob/3739c4a34db6e7abcd8b4faf0db7d5307f37d340/dnf5daemon-server/transaction.hpp#L30
// -- Only a subset of RpmTransactionItemActions is used in dnfdaemon-server.
#[derive(Debug, PartialEq)]
pub enum TransactionAction {
    Install,
    Upgrade,
    Downgrade,
    Reinstall,
    Remove,
    Replaced,
    Unknown(String),
}

impl From<String> for TransactionAction {
    /// Convert a string action into a TransactionAction enum
    fn from(action: String) -> Self {
        match action.to_uppercase().as_str() {
            "INSTALL" => TransactionAction::Install,
            "UPGRADE" => TransactionAction::Upgrade,
            "DOWNGRADE" => TransactionAction::Downgrade,
            "REINSTALL" => TransactionAction::Reinstall,
            "REMOVE" => TransactionAction::Remove,
            "REPLACED" => TransactionAction::Replaced,
            _ => TransactionAction::Unknown(action),
        }
    }
}

// endregion: --- Enums

// region:    --- TransactionMember
/// struct representing a member of a transaction
#[derive(Debug)]
pub struct TransactionMember {
    pub action: TransactionAction,
    pub reason: String,
    pub nevra: String,
    pub sub_action: Option<String>,
}

impl TransactionMember {
    ///New TransactionMember from action, reason and tx_pkg hashmap return my Goal.resolve method
    pub fn from(action: String, reason: String, tx_pkg: TransactionPackage) -> Self {
        let sub_reason = String::try_from(tx_pkg.get("reason").unwrap().to_owned()).unwrap();
        let full_nevra = String::try_from(tx_pkg.get("full_nevra").unwrap().to_owned()).unwrap();
        let sub_action = if sub_reason == "None" || sub_reason == *reason {
            None
        } else {
            Some(sub_reason)
        };
        Self {
            action: action.into(),
            reason,
            nevra: full_nevra,
            sub_action,
        }
    }
}

// endregion: --- TransactionMember

// region:    --- TransactionResult
/// struct representing the result of a transaction
#[derive(Debug)]
pub struct TransactionResult {
    pub tx_members: Vec<TransactionMember>,
    pub result_code: u32,
}

impl TransactionResult {
    /// Create a TransactionResult from a vector of TransactionItems and a result code
    pub fn from(txmbrs: Vec<(TransactionItem)>, result_code: u32) -> Option<Self> {
        let mut members: Vec<TransactionMember> = Vec::new();
        for (_, action, reason, _, tx_pkg) in txmbrs {
            let tx_mbr = TransactionMember::from(action.to_string(), reason.to_string(), tx_pkg.to_owned());
            members.push(tx_mbr);
        }
        Some(Self {
            tx_members: members,
            result_code,
        })
    }

    /// Check if the transaction was successful
    pub fn is_successful(&self) -> bool {
        self.result_code == 0
    }

    /// Show the transaction members
    pub fn show(&self) {
        for mbr in &self.tx_members {
            println!("->> {mbr:?}");
        }
    }
}
// endregion: --- TransactionResult

// region:    --- Transaction
/// struct representing a DNF transaction
pub struct Transaction<'a> {
    dnf_daemon: &'a DnfDaemon,
    transaction_result: Option<TransactionResult>,
}

impl<'a> Transaction<'a> {
    /// Create a new Transaction instance
    pub fn new(dnf_daemon: &'a DnfDaemon) -> Self {
        Self {
            dnf_daemon,
            transaction_result: None,
        }
    }
    /// Install packages in the transaction
    pub async fn install(&self, pkgs: impl AsRef<Vec<String>>) -> Result<()> {
        let options: Options = HashMap::new();
        self.dnf_daemon.rpm.install(pkgs.as_ref(), options).await.ok();
        Ok(())
    }

    /// Remove packages in the transaction
    pub async fn remove(&self, pkgs: impl AsRef<Vec<String>>) -> Result<()> {
        let options: Options = HashMap::new();
        self.dnf_daemon.rpm.remove(pkgs.as_ref(), options).await.ok();
        Ok(())
    }

    /// Update packages in the transaction
    pub async fn update(&self, pkgs: impl AsRef<Vec<String>>) -> Result<()> {
        let options: Options = HashMap::new();
        self.dnf_daemon.rpm.upgrade(pkgs.as_ref(), options).await.ok();
        Ok(())
    }

    /// Reinstall packages in the transaction
    pub async fn reinstall(&self, pkgs: impl AsRef<Vec<String>>) -> Result<()> {
        let options: Options = HashMap::new();
        self.dnf_daemon.rpm.reinstall(pkgs.as_ref(), options).await.ok();
        Ok(())
    }
    /// Resolve the transaction
    pub async fn resolve(&mut self) -> Result<()> {
        let options: Options = HashMap::new();

        if let Ok(rc) = self.dnf_daemon.goal.resolve(options).await {
            self.transaction_result = TransactionResult::from(rc.0, rc.1);
            if let Some(result) = &self.transaction_result
                && !result.is_successful()
            {
                let msgs = self.dnf_daemon.goal.get_transaction_problems_string().await;
                match msgs {
                    Ok(err_msgs) => {
                        return Err(Error::TransactionNotResolved(err_msgs.join("\n")));
                    }
                    Err(_) => {
                        return Err(Error::TransactionNotResolved(
                            "Unknown error during transaction resolution".to_string(),
                        ));
                    }
                }
            }
        };
        Ok(())
    }
    /// Execute the transaction
    pub async fn execute(&mut self) -> Result<()> {
        let options: Options = HashMap::new();
        if let Some(result) = &self.transaction_result
            && result.is_successful()
        {
            // everything is Ok, do transaction
            let _rc = self.dnf_daemon.goal.do_transaction(options.clone()).await.ok();
        }
        Ok(())
    }

    /// Show the transaction result
    pub fn show(&self) {
        if let Some(result) = &self.transaction_result {
            result.show();
        }
    }
}

// endregion: --- Transaction

// region:    --- Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_action_from_string() {
        assert_eq!(
            TransactionAction::from("Install".to_string()),
            TransactionAction::Install
        );
        assert_eq!(
            TransactionAction::from("Upgrade".to_string()),
            TransactionAction::Upgrade
        );
        assert_eq!(
            TransactionAction::from("Downgrade".to_string()),
            TransactionAction::Downgrade
        );
        assert_eq!(
            TransactionAction::from("Reinstall".to_string()),
            TransactionAction::Reinstall
        );
        assert_eq!(TransactionAction::from("Remove".to_string()), TransactionAction::Remove);
        assert_eq!(
            TransactionAction::from("Replaced".to_string()),
            TransactionAction::Replaced
        );
        assert_eq!(
            TransactionAction::from("invalid".to_string()),
            TransactionAction::Unknown("invalid".to_string())
        );
    }

    #[test]
    fn transaction_member_from() {
        use std::collections::HashMap;
        use zbus::zvariant::Value;

        let mut tx_pkg = HashMap::new();
        let value = Value::new("user");
        let ow = Value::new("user").try_to_owned().unwrap();
        tx_pkg.insert("reason".to_string(), Value::new("user").try_to_owned().unwrap());
        tx_pkg.insert(
            "full_nevra".to_string(),
            Value::new("package-1.0-1.x86_64").try_to_owned().unwrap(),
        );

        let member = TransactionMember::from("Install".to_string(), "user".to_string(), tx_pkg);
        assert_eq!(member.action, TransactionAction::Install);
        assert_eq!(member.reason, "user");
        assert_eq!(member.nevra, "package-1.0-1.x86_64");
        assert_eq!(member.sub_action, None);

        // Test with sub_reason
        let mut tx_pkg2 = HashMap::new();
        tx_pkg2.insert("reason".to_string(), Value::new("dependency").try_to_owned().unwrap());
        tx_pkg2.insert(
            "full_nevra".to_string(),
            Value::new("dep-2.0-1.x86_64").try_to_owned().unwrap(),
        );

        let member2 = TransactionMember::from("Install".to_string(), "user".to_string(), tx_pkg2);
        assert_eq!(member2.sub_action, Some("dependency".to_string()));
    }

    #[test]
    fn transaction_result_from_and_methods() {
        use std::collections::HashMap;
        use zbus::zvariant::Value;

        let mut tx_pkg = HashMap::new();
        tx_pkg.insert("reason".to_string(), Value::from("user").try_into_owned().unwrap());
        tx_pkg.insert(
            "full_nevra".to_string(),
            Value::from("package-1.0-1.x86_64").try_into_owned().unwrap(),
        );

        let tx_item = (
            "1".to_string(),
            "Install".to_string(),
            "user".to_string(),
            HashMap::new(),
            tx_pkg,
        );
        let tx_items = vec![tx_item];

        let result = TransactionResult::from(tx_items, 0).unwrap();
        assert!(result.is_successful());
        assert_eq!(result.result_code, 0);
        assert_eq!(result.tx_members.len(), 1);

        let failed_result = TransactionResult::from(vec![], 1).unwrap();
        assert!(!failed_result.is_successful());
        assert_eq!(failed_result.result_code, 1);
    }
}

// endregion: --- Unit Tests
