mod dnf;
use dnf::daemon::DnfDaemon;
use dnf::package::get_packages;
use std::error::Error;

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut dnf_daemon = DnfDaemon::new().await;
    dnf_daemon.base.read_all_repos().await.ok();
    let packages = get_packages(&dnf_daemon, &["yum*"]).await;
    for pkg in packages {
        println!("{} - {}", pkg.nevra, pkg.size);
    }
    dnf_daemon.close().await;

    Ok(())
}
