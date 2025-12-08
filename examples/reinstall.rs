/// Example of how to reinstall a package, run the install example first to install the package
use dnf5daemon::transaction::Transaction;
use dnf5daemon::{DnfDaemon, Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(dnf_daemon) = DnfDaemon::default().await {
        let _rc = dnf_daemon.base.read_all_repos().await.ok().unwrap();
        let pkgs: Vec<String> = vec![String::from("0xFFFF")];
        println!("->> Installing packages {:?}", pkgs);
        let mut transaction = Transaction::new(&dnf_daemon);
        transaction.reinstall(pkgs).await.ok();
        match transaction.resolve().await {
            Ok(_) => {
                transaction.show();
                transaction.execute().await?;
            }
            Err(e) => match e {
                Error::TransactionNotResolved(msgs) => {
                    println!("Transaction errors:");
                    println!("{}", msgs);
                }
                _ => {
                    return Err(e);
                }
            },
        }
    } else {
        println!("Could not connect to dnf5daemon-server");
    };
    Ok(())
}
