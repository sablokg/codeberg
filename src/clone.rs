use std::error::Error;
use std::process::Command;

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn clonerep(username: &str, reponame: &str) -> Result<String, Box<dyn Error>> {
    let formatterstr = format!(
        "{}{}/{}{}",
        "ssh://git@codeberg.org/", username, reponame, ".git"
    );

    let _ = Command::new("git")
        .arg("clone")
        .arg(formatterstr)
        .output()
        .expect("command failed");

    Ok("repository has been cloned".to_string())
}
