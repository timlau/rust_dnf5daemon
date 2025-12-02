#[allow(unused)]
#[allow(dead_code)]
use crate::DnfDaemon;
use crate::{Error, Result};
use std::collections::HashMap;
use zbus::zvariant::OwnedValue;

// region:    --- Enums

// from dnf5 source code:
// https://github.com/rpm-software-management/dnf5/blob/3739c4a34db6e7abcd8b4faf0db7d5307f37d340/dnf5daemon-server/transaction.cpp#L30
// https://github.com/rpm-software-management/dnf5/blob/3739c4a34db6e7abcd8b4faf0db7d5307f37d340/dnf5daemon-server/transaction.hpp#L30
// Only a subset of RpmTransactionItemActions is used in dnfdaemon-server.

#[derive(Debug)]
pub enum TransactionAction {
    Install,
    Upgrade,
    Downgrade,
    Reinstall,
    Remove,
    Unknown,
}

impl From<String> for TransactionAction {
    fn from(action: String) -> Self {
        match action.to_uppercase().as_str() {
            "INSTALL" => TransactionAction::Install,
            "UPGRADE" => TransactionAction::Upgrade,
            "DOWNGRADE" => TransactionAction::Downgrade,
            "REINSTALL" => TransactionAction::Reinstall,
            "REMOVE" => TransactionAction::Remove,
            _ => TransactionAction::Unknown,
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
    pub fn from(action: String, reason: String, tx_pkg: HashMap<String, OwnedValue>) -> Self {
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
// -- Transaction Result element type
type TransactionResultElement = (
    String,
    String,
    String,
    HashMap<String, OwnedValue>,
    HashMap<String, OwnedValue>,
);

impl TransactionResult {
    pub fn from(txmbrs: Vec<(TransactionResultElement)>, result_code: u32) -> Option<Self> {
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

    pub fn is_successful(&self) -> bool {
        self.result_code == 0
    }

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
    pub async fn install(&self, pkgs: &Vec<String>) -> Result<()> {
        let options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>> = HashMap::new();
        self.dnf_daemon.rpm.install(pkgs, options.clone()).await.ok();
        Ok(())
    }

    /// Remove packages in the transaction
    pub async fn remove(&self, pkgs: &Vec<String>) -> Result<()> {
        let options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>> = HashMap::new();
        self.dnf_daemon.rpm.remove(pkgs, options.clone()).await.ok();
        Ok(())
    }

    /// Resolve the transaction
    pub async fn resolve(&mut self) -> Result<()> {
        let options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>> = HashMap::new();

        if let Ok(rc) = self.dnf_daemon.goal.resolve(options.clone()).await {
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
        let options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>> = HashMap::new();
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
