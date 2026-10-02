use std::error::Error;

/*
Gaurav Sablok
gsablok@proton.me
*/

#[tokio::main]
pub async fn addcodeberg(
    username: &str,
    repositoryname: &str,
    commitmessage: &str,
) -> Result<String, Box<dyn Error>> {
    let stringcombine = format!("ssh://git@codeberg.org/{}/{}", username, repositoryname);

    let _ = std::process::Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("command failed");

    let _ = std::process::Command::new("git")
        .arg("remote")
        .arg("set-url")
        .arg("origin")
        .arg(stringcombine)
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
        .arg(commitmessage)
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

    Ok("The repository commit has been pushed".to_string())
}
