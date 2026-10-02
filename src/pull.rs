use std::error::Error;
use std::process::Command;

/*
Gaurav Sablok
gsablok@proton.me
*/

#[tokio::main]
pub async fn pulladdress(
    username: &str,
    reponame: &str,
    branch: &str,
) -> Result<String, Box<dyn Error>> {
    let repoaddress = format!(
        "{}{}/{}{}",
        "ssh://git@codeberg.org/", username, reponame, ".git"
    );

    let _ = Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("command failed");

    let _ = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(repoaddress)
        .output()
        .expect("repository not found");

    let _ = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg(branch)
        .output()
        .expect("branch not found");

    Ok("The repository has been pulled".to_string())
}
