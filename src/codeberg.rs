use core::result::Result;
use std::error::Error;

/*
Gaurav Sablok
gsablok@proton.me
*/

/// ssh://git@codeberg.org/gsablok/codeberg.git

#[tokio::main]
pub async fn codeberg(path1: &str, path2: &str) -> Result<String, Box<dyn Error>> {
    let path1_clone = path1;
    let path2_clone = path2;

    let _ = std::process::Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("command failed");

    let _ = std::process::Command::new("git")
        .arg("remote")
        .arg("set-url")
        .arg("origin")
        .arg(path1_clone)
        .output()
        .expect("command failed");

    let _ = std::process::Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("command failed");

    let _ = std::process::Command::new("git")
        .arg("commit")
        .arg("-am")
        .arg(path2_clone)
        .output()
        .expect("command failed");

    let _ = std::process::Command::new("git")
        .arg("add")
        .arg("-f")
        .arg("*")
        .output()
        .expect("command failed");

    let _ = std::process::Command::new("git")
        .arg("push")
        .output()
        .expect("command failed");

    Ok("The codeberg git commit has been pushed".to_string())
}
