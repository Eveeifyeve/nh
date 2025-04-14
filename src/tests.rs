use assert_cmd::Command as AssertCommand;

#[test]
fn rebuild_test() {
    let mut cmd = AssertCommand::cargo_bin("nh").unwrap();
    let tempdir = tempfile::tempdir().unwrap();

    #[cfg(target_os = "macos")]
    {
        let status = std::process::Command::new("nix")
            .args(&["flake", "init", "-t", "nix-darwin"])
            .current_dir(&tempdir)
            .status()
            .expect("Failed to initialize nix flake");

        assert!(status.success());

        cmd.arg("darwin")
            .arg("build")
            .arg("--hostname")
            .arg("simple")
            .arg("--dry")
            .arg("--no-nom")
            .arg("--verbose")
            .arg(tempdir.path())
            .assert()
            .success();
    }

    #[cfg(not(target_os = "macos"))]
    {
        // TODO: Remove the current dir once there is a flake template in nixpkgs for minimal
        // nixos.
        let current_dir = std::env::current_dir().unwrap().to_str().unwrap();
        let status = std::process::Command::new("nix")
            .args(&["flake", "init", "-t", current_dir, "#test-nixos-system"])
            .current_dir(&tempdir)
            .status()
            .expect("Failed to initialize nix flake");

        assert!(status.success());

        cmd.arg("nixos")
            .arg("switch")
            .arg("--hostname")
            .arg("simple")
            .arg("--dry")
            .arg("--no-nom")
            .arg("--verbose")
            .arg(tempdir.path())
            .assert()
            .success();
    }

    #[cfg(not(target_os = "macos"))]
    {
        let status = std::process::Command::new("nix")
            .args(&["flake", "init", "-t", "nix-community/home-manager"])
            .current_dir(&tempdir)
            .status()
            .expect("Failed to initialize nix flake");

        assert!(status.success());

        cmd.arg("home")
            .arg("switch")
            .arg("--hostname")
            .arg("simple")
            .arg("--dry")
            .arg("--no-nom")
            .arg("--verbose")
            .arg(tempdir.path())
            .assert()
            .success();
    }
}
