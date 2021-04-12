#!/usr/bin/env bash
set -e

function log {
    printf "%s" "$@"
}

function usage {
    printf 'Usage: pactool.sh <command>\nWhere <command> is\n\tgenerate - (re)generate all PACs from their respective SVD source files.\n\tpublish  - (re)publish all PACs to crates.io.\n'
}

function generate {
    # Parse any args
    while [[ $# -gt 0 ]]
    do
        key="$1"
        case $key in
            *)
                INPUT_SVD="$1"
                shift
                break   # The rest of the args will go to the handler
                ;;
        esac
    done

    if [ "$(which svd2rust)" == "" ]; then 
        cargo install --force --git https://github.com/gkelly/svd2rust --branch \
            bleeding-edge --rev 2bbb60590096bcb67c91f38bedd1f63f98132abe svd2rust
    fi

    if [ "$(which form)" == "" ]; then 
        cargo install form --version 0.7.0
    fi

    TOP="${PWD}"

    # 
    # Run through a first pass and create skeleton PAC crates for any that are missing.
    #
    svds=()
    if [ "${INPUT_SVD}" != "" ]; then
        # Process the specific svds given
        while IFS='' read -r line; do svds+=("$line"); done < <(find "${TOP}/svd" -name "${INPUT_SVD}")
    elif [ "${FORCE}" == "true" ]; then
        # If forced, process all SVD files
        while IFS='' read -r line; do svds+=("$line"); done < <(find "${TOP}/svd" -name '*.svd')
    else
        # We only process SVDs that git says are new or changed.
        modified_svds=$(git status --porcelain | grep -e ".svd$" | perl -n -e'/\s*(\S*)\s*(\S+)/ && print $2')
        svds=("${modified_svds}")
    fi

    for svd in "${svds[@]}"; do
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

    #
    # Create the crates using svd2rust
    #
    for svd in "${svds[@]}"; do
        echo "SVD: ${svd}"

        CHIP=$(basename "${svd}" .svd)
        chip=$(echo "${CHIP}" | tr '[:upper:]' '[:lower:]')
        xsl=svd/devices/${chip}.xsl

        pushd "${TOP}/pac/${chip}"

        xsltproc "${TOP}/${xsl}" "${svd}" | svd2rust

        rm -rf src/
        form -i lib.rs -o src
        rm lib.rs
        cargo fmt
        rustfmt build.rs

        popd
    done
}

function publish {
    log "Looking for unpublished crates...\n"
    for p in pac/*; do
        pushd "${p}" > /dev/null
        crate_name=$(grep name Cargo.toml | head -1 | perl -n -e'/(\S*)\s*=\s*(\S+)\s*/ && print $2')
        crate_version=$(grep version Cargo.toml | head -1 | perl -n -e'/(\S*)\s*=\s*(\S+)\s*/ && print $2')
        published_version=$(cargo search "${crate_name}" | perl -n -e'/(\S*)\s*=\s*(\S+)\s*/ && print $2')
        if [ "${crate_version}" == "${published_version}" ]; then
            log "Crate ${crate_name} already published at version ${crate_version}.\n"
        else
            echo Publishing "${crate_name}" "${crate_version}"...
            cargo publish
        fi
        popd > /dev/null
    done
}

#
# Parse arguments
#
COMMAND=""
while [[ $# -gt 0 ]]
do
    key="$1"
    case $key in
        -f|--force)
            FORCE=true
            shift
            ;;
        *)
            COMMAND="$1"
            shift
            break   # The rest of the args will go to the handler
            ;;
    esac
done

#
# If no argument specified, print usage info and exit.
#
if [ "${COMMAND}" == "" ]; then
    usage
    exit 1
fi

#
# Handle any commands
#
case ${COMMAND} in
    help)
        usage
        ;;
    generate)
        generate "$@"
        ;;
    publish)
        publish "$@"
        ;;
    *)
        echo "ERROR: Unrecognized command: ${COMMAND}"
        usage
        exit 1
        ;;    
esac
