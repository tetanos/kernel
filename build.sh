#!/bin/sh

cp src/lib.rs src/main.rs
bootimage build
rm src/main.rs
