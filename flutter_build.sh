#! /usr/bin/env bash

cargo build --lib --features flutter
./build.py --flutter --skip-cargo
