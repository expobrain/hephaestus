#!/bin/bash

set -e

### Setup Rust toolchain #######################################################

# $RUST_VERSION is set by .travis.yml
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain="$RUST_VERSION"
export PATH=$PATH:$HOME/.cargo/bin

# if [ "$TRAVIS_JOB_NAME" = "Minimum nightly" ]; then
#     rustup component add clippy
#     rustup component add rustfmt
# fi


# ### Setup python linker flags ##################################################

PYTHON_BINARY="python"
PYTHON_LIB=$($PYTHON_BINARY -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")

export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$PYTHON_LIB:$HOME/rust/lib"

echo "${LD_LIBRARY_PATH}"
