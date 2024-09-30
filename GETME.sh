#!/bin/sh

cd
nix profile install nixpkgs#rustup
rustup default stable
git clone git@github.com:abelr-githubplace/epita_format.git
cd ./epita_format/
cargo b -rq
cp ./target/release/epita_format ~/afs/
cp ./.clang-format ~/afs/
cp ./README ~/afs/
cd ..
rm -rf epita_format/
