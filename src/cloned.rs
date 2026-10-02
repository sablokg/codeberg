use core::option::Option;
use core::result::Result;
use std::error::Error;
use std::process::Command;

/*
Gaurav Sablok
gsablok@proton.me
*/

#[tokio::main]
pub async fn codecloned(name: &str, path: Option<String>) -> Result<String, Box<dyn Error>> {
    if path.clone().unwrap().len() == 0 {
        let namerep = name;
        let _ = Command::new("git")
            .arg("clone")
            .arg(namerep)
            .output()
            .expect("repository not found");
    }

    if path.clone().unwrap().len() != 0 {
        let namerep = name;
        let namecloned = path.unwrap();
        let _ = Command::new("git")
            .arg("clone")
            .arg(namerep)
            .arg(namecloned)
            .output()
            .expect("path not has been found");
    }

    Ok("The repository has been cloned".to_string())
}
