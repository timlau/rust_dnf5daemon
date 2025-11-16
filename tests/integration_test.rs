use dnf5daemon::dnf::daemon::DnfDaemon;
use dnf5daemon::dnf::package::get_packages;

#[tokio::test]
async fn daemon_test() {
    env_logger::init();
    // Check that new session can be opened.
    let mut dnf_daemon = DnfDaemon::new().await;
    assert_eq!(dnf_daemon.is_connected(), true);
    // Check that we can call a method on one of the interfaces
    let rc = dnf_daemon.base.read_all_repos().await.ok().unwrap();
    assert_eq!(rc, true);
    // Check that we can get a get some packages, using the high-level API
    let pattern: Vec<String> = vec!["dnf5*".to_owned()];
    let packages = get_packages(&dnf_daemon, pattern, &"all".to_string()).await;
    for pkg in &packages {
        println!("{:?}", pkg);
    }
    assert!(packages.len() > 0);
    // check that we can manually close the session
    dnf_daemon.close().await.unwrap();
    assert_eq!(dnf_daemon.is_connected(), false);
}
