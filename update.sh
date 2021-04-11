#!/usr/bin/env bash

set -xe

# INSTALL DEPENDENCIES

cargo install --force --git https://github.com/gkelly/svd2rust --branch \
    bleeding-edge --rev 2bbb60590096bcb67c91f38bedd1f63f98132abe svd2rust
cargo install --force --version 0.7.0 form

TOP="${PWD}"

# Run through a first pass and create skeleton PAC crates for any that are missing.
for svd in svd/*\.svd; do
  CHIP=$(basename "${svd}" .svd)
  chip=$(echo "${CHIP}" | tr '[:upper:]' '[:lower:]')
  xsl=svd/devices/${chip}.xsl

  # If the xsl doesn't exist, create one from a template.
  if [ ! -f "${xsl}" ]; then
    cat > "${xsl}" <<-EOF
<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:include href="include/common.xsl"/>
</xsl:stylesheet>
EOF
  fi

  # if the output directory doesn't exist, create one from a template.
  if [ ! -d "${TOP}/pac/${chip}" ]; then
    # Make the PAC directory (and ./src subdirectory)
    mkdir -p "${TOP}/pac/${chip}/src"
    # Create an empty lib.rs
    touch "${TOP}/pac/${chip}/src/lib.rs"
    # Create a Cargo.toml from a template.
    cat > "${TOP}/pac/${chip}/Cargo.toml" <<-EOF
[package]
authors = ["John Terrell <john@coolpeoplenetworks.com>", "Jacob Alexander <haata@kiibohd.com>"]
categories = ["embedded", "hardware-support", "no-std"]
description = "Peripheral access crate for the ${CHIP} microcontroller"
keywords = ["arm", "cortex-m", "${chip}", "svd2rust"]
license = "MIT OR Apache-2.0"
name = "${chip}-pac"
repository = "https://github.com/atsam-rs/atsam-pac"
version = "0.1.0"

[dependencies]
bare-metal = "0.2.4"
cortex-m = "0.6.3"
vcell = "0.1.2"

[dependencies.cortex-m-rt]
optional = true
version = "0.6.13"

[features]
rt = ["cortex-m-rt/device"]

[package.metadata.docs.rs]
features = ["rt"]
EOF
  fi
done

# Create the crates
for svd in svd/*\.svd; do
  CHIP=$(basename "${svd}" .svd)
  chip=$(echo "${CHIP}" | tr '[:upper:]' '[:lower:]')
  xsl=svd/devices/${chip}.xsl

  pushd "${TOP}/pac/${chip}"

  xsltproc "${TOP}/${xsl}" "${TOP}/${svd}" | svd2rust

  rm -rf src/
  form -i lib.rs -o src
  rm lib.rs
  cargo fmt
  rustfmt build.rs

  popd
done
