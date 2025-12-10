use dnf5daemon::package::{Scope, get_packages};
use dnf5daemon::transaction::Transaction;
use dnf5daemon::{DnfDaemon, Error};

#[tokio::test]
async fn daemon_test() {
    // Check that new session can be opened.
    if let Ok(mut dnf_daemon) = DnfDaemon::default().await {
        assert!(dnf_daemon.is_connected());
        // Check that we can call a method on one of the interfaces
        let rc = dnf_daemon.base.read_all_repos().await.ok().unwrap();
        assert!(rc);
        // Check that we can get a get some packages, using the high-level API
        let pattern: Vec<String> = vec![String::from("dnf5*")];
        let packages = get_packages(&dnf_daemon, pattern, Scope::All)
            .await
            .expect("Error in get_packages");
        for pkg in &packages {
            println!("{:?}", pkg);
        }
        assert!(!packages.is_empty());
        // check that we can manually close the session
        dnf_daemon.close().await.unwrap();
        assert!(!dnf_daemon.is_connected());
    } else {
        println!("Error in creating dbus connection");
    };
}

#[tokio::test]
async fn transaction_operations_test() {
    if let Ok(mut dnf_daemon) = DnfDaemon::default().await {
        let mut transaction = Transaction::new(&dnf_daemon);

        // Test install (dry run, don't actually install)
        let packages = vec!["nonexistent-package".to_string()];
        // Note: These operations will fail in resolve/execute if packages don't exist,
        // but we're testing that the methods exist and can be called
        let result = transaction.install(&packages).await;
        assert!(result.is_ok());

        // Test remove
        let result = transaction.remove(&packages).await;
        assert!(result.is_ok());

        // Test update
        let result = transaction.update(&packages).await;
        assert!(result.is_ok());

        // Test reinstall
        let result = transaction.reinstall(&packages).await;
        assert!(result.is_ok());

        // The resolve method should return an error if the transaction is not resolved
        let resolve_result = transaction.resolve().await;
        assert!(matches!(resolve_result, Err(Error::TransactionNotResolved(_))));

        // Test show (should not panic)
        transaction.show();
        dnf_daemon.close().await.unwrap();
    } else {
        println!("Skipping transaction test: cannot connect to dnf5daemon-server");
    }
}
