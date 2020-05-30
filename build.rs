use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_commit_version = if output.status.success() {
        str::from_utf8(&output.stdout).unwrap()
    } else {
        "unknown"
    };
    let output = Command::new("cargo").args(&["-V"]).output().unwrap();
    let compiler_version = if output.status.success() {
        str::from_utf8(&output.stdout).unwrap()
    } else {
        "unknown"
    };
    println!("cargo:rustc-env=GIT_COMMIT_VERSION={}", git_commit_version);
    println!("cargo:rustc-env=COMPILER_VERSION={}", compiler_version);
}
