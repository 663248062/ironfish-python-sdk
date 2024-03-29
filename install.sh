#!/bin/bash
uname_s=$(uname -s)
if [ $uname_s == 'Darwin' ]; then
    cargo build --release
    cp target/release/libpyironfishlib.dylib  pyironfishlib/pyironfishlib.so
else
    cargo build --release
    cp target/release/libpyironfishlib.so  pyironfishlib/pyironfishlib.so
fi