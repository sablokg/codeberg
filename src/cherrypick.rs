use std::error::Error;
use std::process::Command;

/*
Gaurav Sablok
gsablok@proton.me
*/

#[tokio::main]
pub async fn cherrypicking(branchname: &str, cherrypick: &str) -> Result<String, Box<dyn Error>> {
    let _ = Command::new("git")
        .arg("checkout")
        .arg(branchname)
        .output()
        .expect("branch not found");

    let _ = Command::new("git")
        .arg("cherry-pick")
        .arg(cherrypick)
        .output()
        .expect("branch not found");

    let _ = Command::new("git")
        .arg("pull")
        .output()
        .expect("command not found");
    Ok("The specific branch has been cherry picked".to_string())
}
