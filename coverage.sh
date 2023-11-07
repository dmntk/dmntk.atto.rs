#!/usr/bin/env bash

WORKING_DIRECTORY=$(pwd)

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

# build the code
cargo build

# run all tests
cargo test

# run manual tests
./manual_tests.sh

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage

# generate coverage info
grcov . --llvm -s . -t lcov --branch --ignore-not-existing --ignore "*.cargo*" --ignore "*tests*" -o ./target/lcov/lcov.info

# generate coverage report
genhtml -t "dmntk-atto" -q -o ./target/coverage ./target/lcov/lcov.info

# display final message
echo ""
echo "To open the coverage report, go to: file://$WORKING_DIRECTORY/target/coverage/index.html"
echo ""