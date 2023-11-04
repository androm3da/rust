# `hexagon-unknown-none-elf`

**Tier: 3**

Rust for baremetal Hexagon DSPs.

| Target                   | Descriptions                              |
| ------------------------ | ----------------------------------------- |
| hexagon-unknown-none-elf | Hexagon 32-bit (freestanding, hardfloat)  |

## Target maintainers

- Brian Cain, `bcain@quicinc.com`, https://github.com/androm3da

## Requirements

This target is cross-compiled.  There is no support for `std`. There is no
default allocator, but it's possible to use `alloc` by supplying an allocator.

Document the expectations of binaries built for the target. Do they assume
specific minimum features beyond the baseline of the CPU/environment/etc? What
version of the OS or environment do they expect?

By default, code generated with this target should run on Hexagon DSP hardware.

- `-Ctarget-cpu=hexagonv73` adds support for instructions defined up to Hexagon V73.

Functions marked `extern "C"` use the [Hexagon architecture calling convention](https://tbd).

This target generates PIC ELF binaries.

## Building the target

You can build Rust with support for the target by adding it to the `target`
list in `config.toml`:

```toml
[build]
build-stage = 1
host = ["<target for your host>"]
target = ["<target for your host>", "hexagon-unknown-none-elf"]

[target.hexagon-unknown-none-elf]

cc = "hexagon-unknown-none-elf-clang"
cxx = "hexagon-unknown-none-elf-clang++"
linker = "hexagon-unknown-none-elf-clang"
llvm-libunwind = 'in-tree'
```

Replace `<target for your host>` with `x86_64-unknown-linux-gnu` or whatever
else is appropriate for your host machine.

## Building Rust programs

Rust does not yet ship pre-compiled artifacts for this target. To compile for
this target, you will either need to build Rust with the target enabled (see
"Building the target" above), or build your own copy of `core` by using
`build-std` or similar.

## Testing

Since `hexagon-unknown-none-elf` supports a variety of different environments and does
not support `std`, this target does not support running the Rust test suite.

## Cross-compilation toolchains and C code

This target has been tested using `hexagon-sim` from the Hexagon SDK.

`.cargo/config.toml`:
```toml
[target.hexagon-unknown-none-elf]

cc = "hexagon-unknown-none-elf-clang"
cxx = "hexagon-unknown-none-elf-clang++"
linker = "hexagon-unknown-none-elf-clang"
runner = "hexagon-sim FIXME foo bar baz"

[build]
target = ["hexagon-unknown-none-elf"]
rustflags = "-Ctarget-cpu=hexagonv73"
```

FIXME EXAMPLE
