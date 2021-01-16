<div align="center">

# werb

Yet another rusty package manager

</div>

---

## Install

You can either: 

- Run `cargo install werb`
- Download the binary in the releases

## How to use

### Setup

- Run `werb` to update your sources and setup the environnement

### Search packages

- Run `werb search <package>` to check if your package exists

### Install packages

- Run `werb install <package> [-f | --force]` to install it in `~/.werb_bin`

## Publish packages

_**NOTE**: All the published packages has to be in a single tar.gz file !_

### Adding a new package

**WARNING: If you just want to UPDATE an existing package, go to the [editing section](#edit-an-existing-package)**

- **Prerequisties**: You have to have ruby installed

- Fork this repository
- Clone the produced repository with: `git clone git@ŋithub.com:$USERNAME/werb.git` (obviously replace $USERNAME with your github username)
- Run `cd werb/ && chmod 751 add_package.rb`
- Follow the script instructions
- Commit your changes with this message: `:package: Added $PACKAGE_NAME package` (obviously replace $PACKAGE_NAME with your package name)
- Push your changes to your repo
- Open a pull request on this repository with this title: `[PACKAGE] Added $PACKAGE_NAME` (obviously replace $PACKAGE_NAME with your package name)

- Your package should be accepted within one day

### Edit an existing package


- Fork this repository
- Clone the produced repository with: `git clone git@ŋithub.com:$USERNAME/werb.git` (obviously replace $USERNAME with your github username)
- Go in the produced directory
- Edit `packages.json` with your changes
- Commit your changes with this message: `:package: Edited $PACKAGE_NAME package` (obviously replace $PACKAGE_NAME with your package name)
- Push your changes to your repo
- Open a pull request on this repository with this title: `[PACKAGE] Edited $PACKAGE_NAME` (obviously replace $PACKAGE_NAME with your package name)

- Your package should be updated within one day