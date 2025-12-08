/// Example of how to remove an installed package.
use dnf5daemon::transaction::Transaction;
use dnf5daemon::{DnfDaemon, Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    match DnfDaemon::default().await {
        Ok(dnf_daemon) => {
            match dnf_daemon.base.read_all_repos().await {
                Ok(rc) => println!("Read all repos returned: {:?}", rc),
                Err(e) => eprintln!("Warning: read_all_repos failed: {:?}", e),
            }

            let pkgs: Vec<String> = vec![String::from("0xFFFF")];
            println!("->> Removing packages {:?}", pkgs);
            let mut transaction = Transaction::new(&dnf_daemon);
            transaction.remove(pkgs).await.ok();
            match transaction.resolve().await {
                Ok(_) => {
                    transaction.show();
                    transaction.execute().await?;
                }
                Err(e) => match e {
                    Error::TransactionNotResolved(msgs) => {
                        eprintln!("Transaction errors:");
                        eprintln!("{}", msgs);
                    }
                    _ => {
                        return Err(e);
                    }
                },
            }
        }
        Err(_) => {
            eprintln!("Could not connect to dnf5daemon-server");
        }
    }

    Ok(())
}
