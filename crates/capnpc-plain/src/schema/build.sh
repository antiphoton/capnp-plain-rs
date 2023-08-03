#/bin/sh
set -e;
cd `dirname $0`;

cargo build

CAPNPC_RUST=$PWD/../../../target/debug/capnpc-plain
CAPNP_TOOL=$PWD/../../../../third_party/capnproto/c++/bazel-bin/src/capnp/capnp_tool
CAPNP_SRC=$PWD/../../../../third_party/capnproto/c++/src

export RUST_BACKTRACE=1
$CAPNP_TOOL compile -o- -I$CAPNP_SRC schema.capnp | $CAPNPC_RUST
