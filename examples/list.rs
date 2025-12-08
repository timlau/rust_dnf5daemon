/// Example of how to connect to the dnf5daemon and list packages
///
/// This example follows the same pattern as the other examples in this
/// directory: it connects to the daemon using `DnfDaemon::default().await`,
/// ensures repositories are read, and then lists packages matching a simple
/// pattern.
use dnf5daemon::package::{Scope, get_packages};
use dnf5daemon::{DnfDaemon, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Try to connect to the dnf5daemon server using the default configuration
    match DnfDaemon::default().await {
        Ok(dnf_daemon) => {
            // Ensure repositories are loaded (matches other examples' pattern)
            match dnf_daemon.base.read_all_repos().await {
                Ok(rc) => println!("Read all repos returned: {:?}", rc),
                Err(e) => eprintln!("Warning: read_all_repos failed: {:?}", e),
            }

            // Use a simple pattern and list matching packages
            let pattern: Vec<String> = vec![String::from("dnf5*")];
            println!("Searching for packages matching: {:?}", pattern);

            match get_packages(&dnf_daemon, pattern, Scope::All).await {
                Ok(pkgs) => {
                    println!("Found {} packages:", pkgs.len());
                    for pkg in pkgs.iter().take(50) {
                        println!("  {:?}", pkg);
                    }
                    if pkgs.len() > 50 {
                        println!("  ... ({} more results)", pkgs.len() - 50);
                    }
                }
                Err(e) => {
                    eprintln!("Error while getting packages: {:?}", e);
                }
            }
        }
        Err(_) => {
            eprintln!("Could not connect to dnf5daemon-server");
        }
    }

    Ok(())
}
