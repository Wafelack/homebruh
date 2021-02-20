# Homebruh

A GNU/Linux and MacOS package manager.

## Installation

- From cargo: `cargo install bruh`
- From the releases: download the latest binary.
- From source:

```bash
$ git clone git@github.com:Wafelack/homebruh.git
$ cd homebruh/
$ cargo build --release
```

## Roadmap

- [x] Packages creation.
- [x] Local packages installation.
- [x] Local packages uninstallation.
- [x] Remote package sources.
- [x] Remote packages instalation.
- [x] Remote pacakges uninstallation.
- [ ] Packages uprading.

## Usage

### Creating a package

0. The manifest

A package manifest describes the package, it has to be named `bruh.toml`.

It should contain the following keys:

- `name`   : The package name
- `version`: The package version
- `files`  : The folder containing the package content

Optional keys:

- `startup_script`: The script to run before copying files.
- `cleanup_script`: The script to run after copying files.

1. The package directory

The `files` key directory recreates the filesystem ; e.g. if I have a `usr/bin/bar` in my `files` directory, when installed, there will be a `bar` file in `/usr/bin`.

2. Building the package

To build the package, run `bruh build` in the package directory, this will produce a `$NAME-$VERSION.bpkg` file, this is the built package.

## Installing a local package

To install a local package, you will run `bruh install -i $PACKAGE_FILE` with, of course, your package filename instead of `$PACKAGE_FILE`.

:warning: It may need to be run as super user if the packages creates/removes files at write protected locations.

## Uninstalling a local package

To uninstall a local package, you will run `bruh uninstall -i $PACKAGE_FILE`.

:warning: It may need to be run as super user if the packages removes files at write protected locations.

## Synchronizing the packages database

To synchronize the database (and have your package list up to date), run `sudo bruh sync`.

## Installing a remote package

To install a remote package, you will run `bruh install $package_name`.

:warning: It may need to be run as super user if the packages creates/removes files at write protected locations.

## Uninstalling a remote package

To uninstall a remote package, you will run `bruh uninstall $package_name`.

:warning: It may need to be run as super user if the packages removes files at write protected locations.

## Publishing a package

- [Create a package](#creating-a-package).
- Clone the repo `git clone git@github.com:Wafelack/homebruh.git`.
- Go in the produced folder.
- Create a branch for your adding `git checkout -b add-$package` (obviously replace `$package` with your package name)
- Edit `community/packages.list` and add your package name.
- Create the `community/$package.toml` (obviously replace `$package` with your package name) and add the following information in it:
    - `sha256`: Your package hash (for security purposes), you can obtain it by running `sha256sum $package_file` on GNU/Linux.
    - `link`: The link where your package can be downloaded.

- Open a pull request on this repository with a title matching `Adding $repository`.
- Your package should be added under 24 hours.
