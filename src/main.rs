// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]

use dnf5daemon::daemon::DnfDaemon;
use dnf5daemon::package::get_packages;
// use env_logger;
use log::info;
use std::error::Error;

use clap::Parser;

/// Simple program to test the dnf5 dbus app
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
struct Args {
    /// packages to search for
    // #[arg(short, long)]
    patterns: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup logging
    env_logger::init();
    let args = Args::parse();
    if args.patterns.len() > 0 {
        info!("Starting");
        let dnf_daemon = DnfDaemon::new().await;
        dnf_daemon.base.read_all_repos().await.ok();
        let packages = get_packages(&dnf_daemon, &args.patterns).await;
        for pkg in packages {
            println!(" --> Pkg: {} - {}", pkg.nevra, pkg.size);
        }
        info!("Ending");
    }
    Ok(())
}
