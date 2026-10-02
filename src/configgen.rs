use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

/*
Gaurav Sablok
gsablok@proton.me
*/

#[tokio::main]
pub async fn codegen(name: &str, email: &str) -> Result<String, Box<dyn Error>> {
    let nameclone = name;
    let emailclone = email;

    let _ = Command::new("ssh-keygen")
        .arg("-t")
        .arg("ed25519")
        .arg("-a")
        .arg("100")
        .output()
        .expect("keypair not found");

    let keypath = "~/.ssh/id_ed25519.pub";

    let fileopen = File::open(keypath).expect("file not present");
    let fileread = BufReader::new(fileopen);

    let mut vecfile: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not present");
        if !line.starts_with("ssh") {
            continue;
        } else if line.starts_with("ssh") {
            vecfile.push(line);
        }
    }

    let lineprint = vecfile[0].to_string();

    let _ = Command::new("git")
        .arg("config")
        .arg("-global")
        .arg("user.name")
        .arg(nameclone)
        .output()
        .expect("argument not provided");

    let _ = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg(emailclone)
        .output()
        .expect("argument not provided");

    println!("The key for the codeberg account is {}", lineprint);

    Ok("The config has been generated and configured".to_string())
}
