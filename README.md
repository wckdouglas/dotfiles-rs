# dotfiles-rs

[![CI](https://github.com/wckdouglas/dotfiles-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/wckdouglas/dotfiles-rs/actions/workflows/ci.yml)

This is a simple dot files manager for backing up and syncing dot files across machines.

## Installation

```
$ git clone https://github.com/wckdouglas/dotfiles-rs.git
$ cd dotfiles-rs
$ cargo install --path .
```


## Usage 

The dot files (i.e. config files) to be managed are defined in a yaml file (e.g. `data/dotfiles.yaml`).

### Saving the current settings

We can use the `save` subcommand:

```
$ cd dotfiles-rs
$ dotfiles-rs --dotfile-yaml data/dotfiles.yaml save --dest-dir ../dotfiles
[2022-11-13T15:12:50Z INFO  dotfiles_rs] Copied /Users/wckdouglas/.gitconfig to ../dotfiles/.gitconfig
[2022-11-13T15:12:50Z INFO  dotfiles_rs] Copied /Users/wckdouglas/.zshrc to ../dotfiles/.zshrc
[2022-11-13T15:12:50Z INFO  dotfiles_rs] Copied /Users/wckdouglas/.vimrc to ../dotfiles/.vimrc
[2022-11-13T15:12:50Z INFO  dotfiles_rs] Copied /Users/wckdouglas/.gitignore_global to ../dotfiles/.gitignore_global
[2022-11-13T15:12:50Z INFO  dotfiles_rs] Copied /Users/wckdouglas/.ssh/config to ../dotfiles/.ssh/config
[2022-11-13T15:12:50Z INFO  dotfiles_rs] Copied /Users/wckdouglas/.config/zellij/config.kdl to ../dotfiles/.config/zellij/config.kdl
[2022-11-13T15:12:50Z INFO  dotfiles_rs] Copied /Users/wckdouglas/.config/alacritty/alacritty.yml to ../dotfiles/.config/alacritty/alacritty.yml
[2022-11-13T15:12:50Z INFO  dotfiles_rs] You can now go to ../dotfiles and create a github repo!
```

This creates a new folder `../dotfiles`, and it can be versione-controlled by `git`:

```
$ tree -a -L 2 ../dotfiles
../dotfiles
├── .config
│   ├── alacritty
│   └── zellij
├── .gitconfig
├── .gitignore_global
├── .ssh
│   └── config
├── .vimrc
└── .zshrc
```

Create a github repo:

```
$ cd ../dotfiles
$ git init -b main
$ git add *
$ git commit -am "Adding dot files"
$ git remote add origin <REMOTE_URL>
$ git push -u origin main
```


### Install the settings on a new machine

You will have to provide an url to the `dotfiles` repo (`<REMOTE_URL>`) that you created from the `save` step (e.g. `https://github.com/wckdouglas/dotfiles`)

```
$ cd dotfiles-rs
$ dotfiles-rs --dotfile-yaml data/dotfiles.yaml install --url https://github.com/wckdouglas/dotfiles  --ssh-key ~/.ssh/id_ecdsa
[2022-11-13T15:51:19Z INFO  dotfiles_rs] Cloning git@github.com:wckdouglas/dotfiles.git into /Users/wckdouglas/dotfiles
[2022-11-13T15:51:20Z INFO  dotfiles_rs] Clone complete
[2022-11-13T15:51:20Z INFO  dotfiles_rs] Copied /Users/wckdouglas/dotfiles/.ssh/config to /Users/wckdouglas/.ssh/config
[2022-11-13T15:51:20Z INFO  dotfiles_rs] Copied /Users/wckdouglas/dotfiles/.config/alacritty/alacritty.yml to /Users/wckdouglas/.config/alacritty/alacritty.yml
[2022-11-13T15:51:20Z INFO  dotfiles_rs] Copied /Users/wckdouglas/dotfiles/.vimrc to /Users/wckdouglas/.vimrc
[2022-11-13T15:51:20Z INFO  dotfiles_rs] Copied /Users/wckdouglas/dotfiles/.gitignore_global to /Users/wckdouglas/.gitignore_global
[2022-11-13T15:51:20Z INFO  dotfiles_rs] Copied /Users/wckdouglas/dotfiles/.gitconfig to /Users/wckdouglas/.gitconfig
[2022-11-13T15:51:20Z INFO  dotfiles_rs] Copied /Users/wckdouglas/dotfiles/.zshrc to /Users/wckdouglas/.zshrc
[2022-11-13T15:51:20Z INFO  dotfiles_rs] Copied /Users/wckdouglas/dotfiles/.config/zellij/config.kdl to /Users/wckdouglas/.config/zellij/config.kdl
```

:warning: You may need to create a [ecdsa ssh key](https://exerror.com/youre-using-an-rsa-key-with-sha-1-which-is-no-longer-allowed-please-use-a-newer-client-or-a-different-key-type/) first `ssh-keygen -t ecdsa -b 521 -C "your_email@example.com"` if you see an error like this:

```
ERROR: You're using an RSA key with SHA-1, which is no longer allowed. Please use a newer client or a different key type.
```
