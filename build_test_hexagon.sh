#!/bin/bash -ex

export CC_hexagon_unknown_linux_musl=hexagon-unknown-linux-musl-clang
export CC_hexagon_unknown_none_elf=hexagon-unknown-none-elf-clang
#HOST_TOOLCHAIN=/pkg/qct/software/llvm/build_tools/clang+llvm-15.0.0-linux-x86_64-sles11.3
HOST_TOOLCHAIN=/pkg/qct/software/llvm/build_tools/clang+llvm-16.0.0-x86_64-linux-gnu-ubuntu-18.04/
#export PATH=${PWD}:/pkg/qct/software/gnu/gcc/6.3.0/bin:/pkg/qct/software/python/2.7.12/bin:/pkg/qct/software/llvm/build_tools/llvm38_160329/bin:${PATH}
export PATH=${PWD}:/pkg/qct/software/gnu/gcc/6.3.0/bin:${HOST_TOOLCHAIN}/bin:${PATH}

#export PATH=${PATH}:/pkg/qct/software/llvm/build_tools/clang+llvm-16.0.0-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/bin
#export PATH=${PATH}:/local/mnt/workspace/install/clang+llvm-16.0.5-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/bin
#export PATH=${PATH}:/local/mnt/workspace/install/clang+llvm-17.0.0-rc3-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/bin
export PATH=${PATH}:/local/mnt/workspace/install/clang+llvm-18.1.2-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/bin

export CARGO_TARGET_HEXAGON_UNKNOWN_LINUX_MUSL_RUNNER="qemu-hexagon -L /local/mnt/workspace/install/clang+llvm-18.1.2-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/target/hexagon-unknown-linux-musl/usr"


#/pkg/qct/software/llvm/build_tools/clang+llvm-16.0.0-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/target/hexagon-unknown-linux-musl/
# hack to get around bug, during bootstrap, something
# throws out $CFG_DEFAULT_LINKER from the env and rustc
# tries to use `cc`.
#ln -sf /pkg/qct/software/gnu/gcc/6.3.0/bin/g++ cc

export CC=gcc
export CXX=g++
export CC=clang
export CXX=clang++
#export LD=g++
#export LD=/pkg/qct/software/gnu/gcc/6.3.0/bin/g++
export CFG_DEFAULT_LINKER=clang++
export RUST_BACKTRACE=1

bsub_() {
#   bsub -I -o run_$(date +"%d_%b_%H%M%S").log -R 'select[ubuntu20_llvm] && rusage[mem=12288]' $*
    \time $*
}

build() {
    bsub_ ./x.py build \
        --stage 1 \
        -vv \
        --config config_hex_linux_and_unknown.toml \

    bsub_ ./x.py dist \
        --stage 1 \
        -vv \
        --config config_hex_linux_and_unknown.toml \

    bsub_ ./x.py install \
        --stage 1 \
        -vv \
        --config config_hex_linux_and_unknown.toml \

}

run_tests() {
    bsub_ ./x.py test \
        --stage 1 \
        -vv \
        --config config_hex_linux_and_unknown.toml \
        --exclude src/tools/tidy

}

set -euo pipefail

build 2>&1 | tee build.log
run_tests 2>&1 | tee test.log

