#!/bin/sh
sed -i '/extern crate aoc;/a\
extern crate captures_at;\
use captures_at::CapturesAtExt;' soln.rs
sed -i '/\[dependencies.libaoc\]/i\
[dependencies.libcapturesat]\
path = "../libcapturesat"\
' Cargo.toml
