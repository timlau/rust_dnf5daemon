use dnf5daemon::DnfDaemon;
use dnf5daemon::package::{Scope, get_packages};

#[tokio::test]
async fn daemon_test() {
    env_logger::init();
    // Check that new session can be opened.
    if let Ok(mut dnf_daemon) = DnfDaemon::default().await {
        assert!(dnf_daemon.is_connected());
        // Check that we can call a method on one of the interfaces
        let rc = dnf_daemon.base.read_all_repos().await.ok().unwrap();
        assert!(rc);
        // Check that we can get a get some packages, using the high-level API
        let pattern: Vec<String> = vec!["dnf5*".to_owned()];
        let packages = &get_packages(&dnf_daemon, &pattern, Scope::All)
            .await
            .expect("Error in get_packages");
        for pkg in packages {
            println!("{:?}", pkg);
        }
        assert!(packages.is_empty());
        // check that we can manually close the session
        dnf_daemon.close().await.unwrap();
        assert!(!dnf_daemon.is_connected());
    } else {
        println!("Error in creating dbus connection");
    };
}
