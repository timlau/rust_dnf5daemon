use dnf5daemon::dnf::daemon::DnfDaemon;
use dnf5daemon::dnf::package::get_packages;

#[tokio::test]
async fn daemon_test() {
    env_logger::init();
    let mut dnf_daemon = DnfDaemon::new().await;
    assert_eq!(dnf_daemon.is_connected(), true);
    let rc = dnf_daemon.base.read_all_repos().await.ok().unwrap();
    assert_eq!(rc, true);
    let pattern: Vec<String> = vec!["dnf5*".to_owned()];
    let packages = get_packages(&dnf_daemon, pattern, &"all".to_string()).await;
    for pkg in &packages {
        println!("{:?}", pkg);
    }
    assert!(packages.len() > 0);
    dnf_daemon.close().await.unwrap();
    assert_eq!(dnf_daemon.is_connected(), false);
}
