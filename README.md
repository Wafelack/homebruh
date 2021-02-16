# Homebruh

A GNU/Linux and MacOS package manager.

## Installation

- From source:

```bash
$ git clone git@github.com:Wafelack/homebruh.git
$ cd homebruh/
$ cargo build --release
```

## Roadmap

- [x] Packages creation.
- [x] Local packages installation.
- [ ] Remote package sources.
- [ ] Remote packages instalation.
- [ ] Packages publishing.

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

## Installing local package

To install a local package, you will run `bruh install -i $PACKAGE_FILE` with, of cours, your package filename instead of `$PACKAGE_FILE`.

:warning: It may need to be run as super user if the packages creates/removes file at write protected locations.
