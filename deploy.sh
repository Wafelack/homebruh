#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-incversion &> /dev/null;then
  echo "fatal: cargo-incversion could not be found. aborting."
  exit
fi

if ! command -v stoml &> /dev/null;then
  echo "fatal: stoml could not be found. aborting."
  exit
fi

if ! command -v cargo &> /dev/null;then
  echo "fatal: cargo could not be found. aborting."
  exit
fi

if ! command -v git &> /dev/null;then
  echo "fatal: git could not be found. aborting."
  exit
fi

if [ -f "./Cargo.toml" ];then
else
  echo "fatal: could not find `Cargo.toml` in the current directory"
  exit
fi

if [[ $1 == "major" ]];then
  cargo-incversion -M
elif [[ $1 == "minor" ]];then
  cargo-incversion -m
else
  echo "No incrementation specified. Incrementing patch"
  cargo-incversion -p
fi

archive=""

name=$(stoml Cargo.toml package.name)
version=$(stoml Cargo.toml package.version)

if [ -z $2 ];then
  echo "No archive specified. Using default archive name"
  archive="${name}.tar.gz"
else
  archive=$2

if [ -f $2 ];then
  rm $2 -f
fi

cargo build --release

executable="./target/release/$name"


tar -czf $tarchive $executable
git add .
git commit -m  "deploy: automatic deploy commit for version $version"
git push