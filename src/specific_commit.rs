use std::error::Error;
use std::process::Command;

/*
Gaurav Sablok
codeprog@icloud.com
*/

#[tokio::main]
pub async fn specific_commit_add(
    commitsha: &str,
    branchname: &str,
) -> Result<String, Box<dyn Error>> {
    let _ = Command::new("git")
        .arg("fetch")
        .arg("origin")
        .output()
        .expect("command failed");
    let _ = Command::new("git")
        .arg("checkout")
        .arg(commitsha)
        .output()
        .expect("command failed");

    let _ = Command::new("git")
        .arg("switch")
        .arg("--detach")
        .arg(commitsha)
        .output()
        .expect("command failed");

    let _ = Command::new("git")
        .arg("switch")
        .arg("-c")
        .arg(branchname)
        .arg(commitsha)
        .output()
        .expect("command failed");

    let _ = Command::new("git")
        .arg("pull")
        .output()
        .expect("command not found");

    Ok("The specific commit has been pulled".to_string())
}
