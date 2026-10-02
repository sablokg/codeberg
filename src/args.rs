pub(crate) use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "codeberg",
    version = "1.0",
    about = "codeberg commit development software.
       ************************************************
       Author Gaurav Sablok,
       Email: gsablok@proton.me
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Push all the commit to the codeberg
    Codeberg {
        /// codeberg repository
        codebergpush: String,
        /// commit message
        commitmessage: String,
    },
    /// Clone the repository
    CodeClone {
        /// name of the repository
        namerep: String,
        /// path of the repository
        pathrep: Option<String>,
    },
    /// Generate Keypair
    KeyGenerate {
        /// name of the user
        name: String,
        ///email of the use
        email: String,
    },
    /// Commitpush
    Commitpush {
        /// push the commit to the repository
        usernameadd: String,
        /// repository name
        repositorynameadd: String,
        /// commit message
        commitmessage: String,
        /// threads for the parallel execution
        threads: String,
    },
    /// Pull the repo with the branch
    Pull {
        /// username of the user
        username: String,
        /// repoaddress of the user
        repoaddress: String,
        /// branch of the repository
        branch: String,
    },
    /// Commit speicific branch
    CommitBranch {
        /// Commit branch
        commitshar: String,
        /// branch name
        branchnamer: String,
    },
    /// Cherrypicking specific commit
    CherryPick {
        /// branchname for the cherry picking
        branchnamer: String,
        /// commit of the branch
        committag: String,
    },
    /// Push all the commit to the codeberg
    Username {
        /// codeberg name
        codebergname: String,
        /// repository name
        repository: String,
        /// commit message
        commitmessage: String,
    },
    /// Clone a repo just by username and reponame
    RepoClone {
        /// name of the username
        namerep: String,
        /// name of the repository
        repname: String,
    },
}
