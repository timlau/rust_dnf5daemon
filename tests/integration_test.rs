use dnf5daemon::dnf::daemon::DnfDaemon;
use dnf5daemon::dnf::package::get_packages;

#[tokio::test]
async fn daemon_test() {
    let mut dnf_daemon = DnfDaemon::new().await;
    assert_eq!(dnf_daemon.connected, true);
    let rc = dnf_daemon.base.read_all_repos().await.ok().unwrap();
    assert_eq!(rc, true);
    let packages = &get_packages(&dnf_daemon, &["yum*"]).await;
    for pkg in packages {
        println!("{} - {}", pkg.nevra, pkg.size);
    }
    assert!(packages.len() > 0);
    dnf_daemon.close().await;
    assert_eq!(dnf_daemon.connected, false);
}
