mod args;
mod codeberg;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use codeberg::codeberg;
mod cloned;
use cloned::codecloned;
mod configgen;
use configgen::codegen;
mod add;
use crate::add::addcodeberg;
mod pull;
use crate::pull::pulladdress;
mod specific_commit;
use crate::specific_commit::specific_commit_add;
mod cherrypick;
use crate::cherrypick::cherrypicking;
mod username;
use crate::username::codeberg_username;
use vornix_banner::{Banner, BuiltinFont, Style, rgb};
mod clone;
use crate::clone::clonerep;

/*
Gaurav Sablok
gsablok@proton.me
*/

#[tokio::main]
async fn main() {
    let style = Style::new().fg(rgb(255, 100, 20)).bold();

    let mut banner = Banner::new("Codeberg")
        .with_builtin_font(BuiltinFont::Block)
        .with_style(style)
        .centered(true);

    banner.display().unwrap();
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Codeberg {
            codebergpush,
            commitmessage,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(4usize)
                .build()
                .unwrap();
            pool.install(|| {
                let codeberg_run = codeberg(codebergpush, commitmessage).unwrap();
                println!("The git commit has been pushed:{}\n", codeberg_run);
            });
        }
        Commands::CodeClone { namerep, pathrep } => {
            let commandrun = codecloned(namerep, Some(pathrep.clone().unwrap())).unwrap();
            println!(
                "The command has finished and the repository has been cloned: {}",
                commandrun
            );
        }
        Commands::KeyGenerate { name, email } => {
            let commandrun = codegen(name, email).unwrap();
            println!("The key binding of the codegen are: {}", commandrun);
        }
        Commands::Commitpush {
            usernameadd,
            repositorynameadd,
            commitmessage,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let codebergrun =
                    addcodeberg(usernameadd, repositorynameadd, commitmessage).unwrap();
                println!("The commit has been pushed:{}", codebergrun);
            });
        }
        Commands::Pull {
            username,
            repoaddress,
            branch,
        } => {
            let threads: usize = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                let command = pulladdress(username, repoaddress, branch).unwrap();
                println!(
                    "The command has finished and the repository has been pulled: {}",
                    command
                );
            });
        }
        Commands::CommitBranch {
            commitshar,
            branchnamer,
        } => {
            let threads: usize = 0usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                let command = specific_commit_add(commitshar, branchnamer).unwrap();
                println!(
                    "The commit from the speicific branch with the specific sha has been pulled:{}",
                    command
                );
            });
        }
        Commands::CherryPick {
            branchnamer,
            committag,
        } => {
            let nthreads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(nthreads)
                .build()
                .unwrap();
            pool.install(|| {
                let command = cherrypicking(branchnamer, committag).unwrap();
                println!(
                    "The commit tag has been cherry picked from the specific branch: {}",
                    command
                );
            });
        }
        Commands::Username {
            codebergname,
            repository,
            commitmessage,
        } => {
            let nthreads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(nthreads)
                .build()
                .unwrap();
            pool.install(|| {
                let command = codeberg_username(codebergname, repository, commitmessage).unwrap();
                println!("The commit has been pushed: {}", command);
            });
        }
        Commands::RepoClone { namerep, repname } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .unwrap();
            pool.install(|| {
                let command = clonerep(namerep, repname).unwrap();
                println!("The command has finished:{}", command);
            })
        }
    }
}
