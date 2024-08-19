#! /usr/bin/env bash

# install clang-devel

rm ./flutter/lib/generated_bridge.*

flutter_rust_bridge_codegen --rust-input ./src/flutter_ffi.rs --dart-output ./flutter/lib/generated_bridge.dart
