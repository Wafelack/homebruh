# yarpm

Yet another rusty package manager

## Install

You can either: 

- Run `cargo install werb`
- Download the binary in the releases

## How to use

### Setup

- Run `werb` once to create all the required files
- Edit `/etc/yarpm.sources` with your sources (one per line)

### Search packages

- Run `yarpm search <package>` to check if your package exists

### Install packages

- Run `yarpm install <package>` to install it in `/etc/yarpm/bin`

### Update packages

- Run `yarpm upgrade`

## Publish packages

- To publish packages they have to be available on the internet and they have to be represented by a single `tar.gz` file
- Publish your package on the internet (a github repo or a server)
- Add the link to your `/etc/yarpm.sources`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contributing rules