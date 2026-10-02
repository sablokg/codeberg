use core::result::Result;
use std::error::Error;

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn codeberg_username(path1: &str, path2: &str, path3: &str) -> Result<String, Box<dyn Error>> {
    let path1_path2 = format!("{}{}/{}{}", "ssh://git@codeberg.org/", path1, path2, ".git");
    let _ = std::process::Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("command failed");

    let _ = std::process::Command::new("git")
        .arg("remote")
        .arg("set-url")
        .arg("origin")
        .arg(path1_path2)
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
        .arg(path3)
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
