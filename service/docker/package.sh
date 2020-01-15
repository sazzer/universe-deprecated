#!/bin/sh

rm -rf /universe/crates/*/src
rm -rf /universe/crates/*/tests

for i in crates/*; do
  echo ===== $i =====
  mkdir $i/src
  touch $i/src/lib.rs
  echo 'fn main() {println!("Wrong");}' > $i/src/main.rs
done
