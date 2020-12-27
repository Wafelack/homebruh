# yarpm

Yet another rusty package manager

## Install

- Run `cargo install yarpm` to install program

## How to use

- Run `yarpm` once to create all the required files
- Edit `/etc/yarpm.sources` with your sources (one per line)
- Run `cargo search <package>` to check if your package exists
- Run `cargo install <package>` to install it in `/etc/yarpm/bin`

## Publish packages

- Launch a webserver
- Add your compressed files at the server root (NOTICE: Files have to be compressed using the gunzip algorithms (Use `tar -czf <name> <folder>`))
- Put your server link into `/etc/yarpm.sources`
- You can now install your packages