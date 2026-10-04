use std::process::Command;

#[test]
fn help_flag_works() {
    let output1 = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["-h", "print"])
        .output()
        .expect("failed to launch rgrep");

    let output2 = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["--help", "print"])
        .output()
        .expect("failed to launch rgrep");

    assert!(output1.status.success());
    assert!(output2.status.success());
}

#[test]
fn version_flag_works() {
    let output1 = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["-v", "print"])
        .output()
        .expect("failed to launch rgrep");

    let output2 =  Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["--version", "print"])
        .output()
        .expect("failed to launch rgrep");

    assert!(output1.status.success());
    assert!(output2.status.success());
}

#[test]
fn case_flag_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["-i", "print"])
        .output()
        .expect("failed to launch rgrep");

    assert!(output.status.success());
}

#[test]
fn default_path_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["print"])
        .output()
        .expect("failed to launch rgrep");

    assert!(output.status.success());
}

#[test]
fn provided_path_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["print", "."])
        .output()
        .expect("failed to launch rgrep");

    assert!(output.status.success());
}

#[test]
fn invalid_flag_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["-t", "print", "."])
        .output()
        .expect("failed to launch rgrep");

    assert!(!output.status.success());
}

#[test]
fn invalid_path_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .args(["print", "./kjsdf"])
        .output()
        .expect("failed to launch rgrep");

    assert!(!output.status.success());
}

#[test]
fn no_args_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_rgrep"))
        .output()
        .expect("failed to launch rgrep");

    assert!(!output.status.success());
}