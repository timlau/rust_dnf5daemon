#![allow(unreachable_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]

use dnf5daemon::daemon::DnfDaemon;
use dnf5daemon::package::get_packages;
// use env_logger;
use log::info;
use std::env;
use std::error::Error;

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup logging
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        info!("Starting");
        let dnf_daemon = DnfDaemon::new().await;
        dnf_daemon.base.read_all_repos().await.ok();
        let packages = get_packages(&dnf_daemon, &[args[1].as_str()]).await;
        for pkg in packages {
            println!(" --> Pkg: {} - {}", pkg.nevra, pkg.size);
        }
        info!("Ending");
    }

    Ok(())
}
