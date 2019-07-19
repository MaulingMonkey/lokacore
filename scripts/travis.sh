#!/bin/bash

set -e

rustup component add clippy
cargo clippy

if [[ "$TARGET" != "" ]]; then rustup target install $TARGET; fi

if [[ "$TARGET" == "wasm32-"* && "$TARGET" != "wasm32-wasi" ]]; then
  cargo-web --version || cargo install cargo-web
  cargo web build $FLAGS --target=$TARGET
  cargo web test  $FLAGS --target=$TARGET

elif [[ "$TARGET" == *"-linux-android"* ]]; then
  export PATH=/usr/local/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH
  pushd linux-android
    cargo build --target=$TARGET
    # Don't test, can't run android emulators successfully on travis currently
  popd

elif [[ "$TARGET" == *"-apple-ios" || "$TARGET" == "wasm32-wasi" ]]; then
  cargo build --target=$TARGET $FLAGS
  # Don't test
  #   iOS simulator setup/teardown is complicated
  #   cargo-web doesn't support wasm32-wasi yet, nor can wasm-pack test specify a target

elif [[ "$TARGET" != "" ]]; then
  pushd generic-cross
    cargo build --target=$TARGET $FLAGS
    cargo test  --target=$TARGET $FLAGS
  popd

else
  # Push nothing, target host CPU architecture
  cargo build $FLAGS
  cargo test  $FLAGS

fi
