/// Example of how to reinstall a package, run the install example first to install the package
use dnf5daemon::DnfDaemon;
use dnf5daemon::transaction::Transaction;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dnf_daemon = DnfDaemon::default().await?;
    let mut transaction = Transaction::new(&dnf_daemon);

    let packages = vec!["0xFFFF".to_string()];
    transaction.reinstall(&packages).await?;

    transaction.resolve().await?;
    transaction.execute().await?;
    transaction.show();

    Ok(())
}
