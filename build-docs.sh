#!/usr/bin/sh

rm -r docs target/doc
cargo doc --release --no-deps
echo "<meta http-equiv=\"refresh\" content=\"0; url=no_flag_4_u\">" > target/doc/index.html
cp -r target/doc docs
