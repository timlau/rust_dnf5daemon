/// Example of how to list packages matching a
use dnf5daemon::DnfDaemon;
use dnf5daemon::package::{Scope, get_packages};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(dnf_daemon) = DnfDaemon::default().await {
        let rc = dnf_daemon.base.read_all_repos().await.ok().unwrap();
        println!("Read all repos returned: {:?}", rc);
        let pattern: Vec<String> = vec![String::from("dnf5*")];
        let packages = get_packages(&dnf_daemon, pattern, Scope::All)
            .await
            .expect("Error in get_packages");
        for pkg in packages {
            println!("{:?}", pkg);
        }
    } else {
        println!("Could not connect to dnf5daemon-server");
    };

    Ok(())
}
