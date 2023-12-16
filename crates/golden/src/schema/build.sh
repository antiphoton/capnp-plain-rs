#/bin/sh
set -e;
cd `dirname $0`;


TARGET=$PWD

cd ../../..
cargo build
CAPNPC_RUST=$PWD/target/debug/capnpc-plain

cd ../third_party/capnproto/c++
CAPNP_TOOL=$PWD/bazel-bin/src/capnp/capnp_tool

cd src
export RUST_BACKTRACE=1
$CAPNP_TOOL compile -o- -I. ./capnp/test.capnp | $CAPNPC_RUST
mv ./capnp/test_capnp.rs $TARGET
