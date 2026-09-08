#!/usr/bin/env bash
# Cloud Agent bootstrap for Powrush-MMO.
# Installs the system libraries Bevy needs to build, a software-Vulkan
# (lavapipe) + Xvfb stack so the lived-hour client can run headlessly, a
# Rust toolchain new enough for the dependency graph, and warms the build
# cache. Safe to run repeatedly.
set -euo pipefail

echo "== Powrush-MMO install: system dependencies =="
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  pkg-config \
  build-essential \
  libasound2-dev \
  libudev-dev \
  libx11-dev \
  libxkbcommon-x11-dev \
  libwayland-dev \
  libxkbcommon-dev \
  mesa-vulkan-drivers \
  libvulkan1 \
  vulkan-tools \
  libgl1-mesa-dri \
  xvfb \
  x11-utils

echo "== Powrush-MMO install: Rust toolchain =="
# The dependency graph pulls crates that require edition2024 / Rust >= 1.87
# (e.g. coreaudio-sys, uuid, wasip2). The repo default 1.83 is too old, and
# CI uses `stable`, so pin stable here.
rustup toolchain install stable --profile minimal
rustup default stable
rustc --version

echo "== Powrush-MMO install: build core workspace (shared, rsil-identity, client) =="
# Compile lib + test targets so the cache is warm for both `cargo run` and CI.
cargo build --workspace --all-targets

echo "== Powrush-MMO install: complete =="
