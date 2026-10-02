# codeberg

- Rust client for the commit push to the codeberg. 
- Download and install the cargo and then just run with the codeberg git repository as the first argument and the second argument as the commit message.

```
git clone ssh://git@codeberg.org/gsablok/codeberg.git
cd codeberg
cargo build

```


```
                                                                                                                                     
                                                _|_|_|                    _|              _|                                         
                                              _|           _|_|       _|_|_|     _|_|     _|_|_|       _|_|     _|  _|_|     _|_|_|  
                                              _|         _|    _|   _|    _|   _|_|_|_|   _|    _|   _|_|_|_|   _|_|       _|    _|  
                                              _|         _|    _|   _|    _|   _|         _|    _|   _|         _|         _|    _|  
                                                _|_|_|     _|_|       _|_|_|     _|_|_|   _|_|_|       _|_|_|   _|           _|_|_|  
                                                                                                                                 _|  
                                                                                                                             _|_|    
codeberg commit development software.
       ************************************************
       Author Gaurav Sablok,
       Email: gsablok@proton.me
      ************************************************

Usage: codeberg <COMMAND>

Commands:
  codeberg       Push all the commit to the codeberg
  code-clone     Clone the repository
  key-generate   Generate Keypair
  commitpush     Commitpush
  pull           Pull the repo with the branch
  commit-branch  Commit speicific branch
  cherry-pick    Cherrypicking specific commit
  username       Push all the commit to the codeberg
  repo-clone     Clone a repo just by username and reponame
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```
```
codeberg codeberg ssh://git@codeberg.org/gsablok/codeberg.git complete 10
   ____               _          _                           
  / ___|   ___     __| |   ___  | |__     ___   _ __    __ _ 
 | |      / _ \   / _` |  / _ \ | '_ \   / _ \ | '__|  / _` |
 | |___  | (_) | | (_| | |  __/ | |_) | |  __/ | |    | (_| |
  \____|  \___/   \__,_|  \___| |_.__/   \___| |_|     \__, |
                                                       |___/ 

The git commit has been pushed:The codeberg git commit has been pushed

```

### New feature short version of commit push just username and the commit message as a input.

```
codeberg username gsablok codeberg complete

```

### New feature just know the username and the name of the repository to be cloned 

```
codeberg repo-clone gsablok viroencoder

```
