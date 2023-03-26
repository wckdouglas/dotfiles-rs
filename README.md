# dotfiles-rs

[![CI](https://github.com/wckdouglas/dotfiles-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/wckdouglas/dotfiles-rs/actions/workflows/ci.yml)
[![crate](https://img.shields.io/crates/v/dotfiles-rs.svg)](https://crates.io/crates/dotfiles-rs)


This is a simple dot files manager for syncing dot files across machines.

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
$ dotfiles-rs --dotfile-yaml data/dotfiles.yaml save --dest-dir ../dotfiles --dry-run
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.zshrc to ../dotfiles/.zshrc
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.wezterm.lua to ../dotfiles/.wezterm.lua
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.config/nvim/init.lua to ../dotfiles/.config/nvim/init.lua
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.gitconfig to ../dotfiles/.gitconfig
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.vimrc to ../dotfiles/.vimrc
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.ssh/config to ../dotfiles/.ssh/config
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.gitignore_global to ../dotfiles/.gitignore_global
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.config/alacritty/alacritty.yml to ../dotfiles/.config/alacritty/alacritty.yml
[2023-03-26T14:46:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/.config/zellij/config.kdl to ../dotfiles/.config/zellij/config.kdl
[2023-03-26T14:46:43Z INFO  dotfiles_rs] You can now go to ../dotfiles and create a github repo!
```

This creates a new folder `../dotfiles`, and it can be versione-controlled by `git`:

```
$ tree -a -L 2 ../dotfiles
../dotfiles
├── .config
├── .config
│   ├── alacritty
│   │   └── alacritty.yml
│   ├── nvim
│   │   └── init.lua
│   └── zellij
│       └── config.kdl
├── .gitconfig
├── .gitignore_global
├── .ssh
│   └── config
├── .vimrc
├── .zshrc
└── README.md
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


### Applying the settings on a new machine

You will have to provide an url to the `dotfiles` repo (`<REMOTE_URL>`) that you created from the `save` step (e.g. `https://github.com/wckdouglas/dotfiles`)

```
$ cd dotfiles-rs
$ dotfiles-rs --dotfile-yaml data/dotfiles.yaml apply --url git@github.com:wckdouglas/dotfiles  --ssh-key ~/.ssh/id_ecdsa --dry-run
[2023-03-26T14:51:43Z INFO  dotfiles_rs] Cloning git@github.com:wckdouglas/dotfiles into /Users/wckdouglas/dotfiles
[2023-03-26T14:51:43Z INFO  dotfiles_rs] Clone complete
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.wezterm.lua to /Users/wckdouglas/.wezterm.lua
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.vimrc to /Users/wckdouglas/.vimrc
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.config/alacritty/alacritty.yml to /Users/wckdouglas/.config/alacritty/alacritty.yml
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.config/nvim/init.lua to /Users/wckdouglas/.config/nvim/init.lua
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.ssh/config to /Users/wckdouglas/.ssh/config
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.gitignore_global to /Users/wckdouglas/.gitignore_global
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.zshrc to /Users/wckdouglas/.zshrc
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.config/zellij/config.kdl to /Users/wckdouglas/.config/zellij/config.kdl
[2023-03-26T14:51:43Z INFO  dotfiles_rs] [Dry run] Copied /Users/wckdouglas/dotfiles/.gitconfig to /Users/wckdouglas/.gitconfig
```

:warning: You may need to create a [ecdsa ssh key](https://exerror.com/youre-using-an-rsa-key-with-sha-1-which-is-no-longer-allowed-please-use-a-newer-client-or-a-different-key-type/) first `ssh-keygen -t ecdsa -b 521 -C "your_email@example.com"` if you see an error like this:

```
ERROR: You're using an RSA key with SHA-1, which is no longer allowed. Please use a newer client or a different key type.
```

# Docker 

```
$ docker pull ghcr.io/wckdouglas/dotfiles-rs:main
$ docker run  ghcr.io/wckdouglas/dotfiles-rs:main
```

# Debug #

On m1 mac, if you see an error like this:

```
$ cargo run --
... skipping some compilation messages ...
 = note: Undefined symbols for architecture arm64:
            "_iconv", referenced from:
                _git_fs_path_iconv in liblibgit2_sys-2da99193d83f7067.rlib(fs_path.o)
               (maybe you meant: _git_fs_path_iconv_clear, _git_fs_path_iconv_init_precompose , _git_fs_path_iconv )
            "_iconv_close", referenced from:
                _git_fs_path_iconv_clear in liblibgit2_sys-2da99193d83f7067.rlib(fs_path.o)
            "_iconv_open", referenced from:
                _git_fs_path_iconv_init_precompose in liblibgit2_sys-2da99193d83f7067.rlib(fs_path.o)
          ld: symbol(s) not found for architecture arm64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)

```

then, doing this may work:

```
$ LIBRARY_PATH=/Library/Developer/CommandLineTools/SDKs/MacOSX13.1.sdk/usr/include cargo install --path .
```
