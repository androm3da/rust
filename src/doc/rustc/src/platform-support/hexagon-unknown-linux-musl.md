# `hexagon-unknown-linux-musl`

**Tier: 3**

Target for cross-compiling Linux user-mode applications targeting the Hexagon
DSP architecture.

| Target                   | Descriptions                              |
| ------------------------ | ----------------------------------------- |
| hexagon-unknown-linux-musl | Hexagon 32-bit Linux |

## Target maintainers

- [Brian Cain](https://github.com/androm3da), `bcain@quicinc.com`

## Requirements
The target is cross-compiled. This target supports `std`.  By default, code
generated with this target should run on Hexagon DSP hardware.

- `-Ctarget-cpu=hexagonv73` adds support for instructions defined up to Hexagon V73.

Binaries can be run using QEMU user emulation. On Debian-based systems, it should be
sufficient to install the package `qemu-user-static` to be able to run simple static
binaries:

```text
# apt install qemu-user-static
# qemu-hexagon ./hello
```

In order to build linux programs with Rust, you will require a linker capable
of targeting hexagon.  You can use `clang`/`lld` from the [hexagon toolchain
using exclusively public open source repos](https://github.com/quic/toolchain_for_hexagon/releases).

Also included in that toolchain is the C library that can be used when creating
dynamically linked executables.

```text
# ./clang+llvm-99.9-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/bin/qemu-hexagon -L ./clang+llvm-99.9-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/target/hexagon-unknown-linux-musl/usr/ ./hello
```

## Building the target
Because it is Tier 3, rust does not yet ship pre-compiled artifacts for this target.

Therefore, you can build Rust with support for the target by adding it to the
target list in config.toml, a sample configuration is shown below.

```toml
[build]
target = [ "hexagon-unknown-linux-musl"]
docs = false

[target.hexagon-unknown-linux-musl]

cc = "hexagon-unknown-linux-musl-clang"
cxx = "hexagon-unknown-linux-musl-clang++"
linker = "hexagon-unknown-linux-musl-clang"
ar = "hexagon-unknown-linux-musl-ar"
ranlib = "hexagon-unknown-linux-musl-ranlib"
musl-root = "/usr/local/clang+llvm-18.0.0-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/target/hexagon-unknown-linux-musl/usr"
llvm-libunwind = 'in-tree'
qemu-rootfs = "/usr/local/clang+llvm-18.0.0-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/target/hexagon-unknown-linux-musl/usr"
```


## Testing

Currently there is no support to run the rustc test suite for this target.


```text
rustc --target hexagon-unknown-linux-musl your-code.rs
```

## Building Rust programs

The following `.cargo/config` is needed inside any project directory to build
for the Hexagon Linux target:

```toml
[build]
target = "hexagon-unknown-linux-musl"

[target.hexagon-unknown-linux-musl]
linker = "hexagon-unknown-linux-musl-clang"
ar = "hexagon-unknown-linux-musl-ar"
runner = "/usr/bin/env QEMU_LD_PREFIX=/usr/local/clang+llvm-18.0.0-rc1-cross-hexagon-unknown-linux-musl/x86_64-linux-gnu/target/hexagon-unknown-linux-musl/usr qemu-hexagon"
```

