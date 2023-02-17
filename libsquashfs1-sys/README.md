# libsquashfs1-sys

A rust wrapper
for [libsquashfs](https://infraroot.at/projects/squashfs-tools-ng/doxydoc/index.html).
I am very new to this so there are probably many things that can be improved. I
would consider these **bindings unstable** for the time being as I am still
learning how to best use [bindgen](https://github.com/rust-lang/rust-bindgen).

## Usage

This library requires the shared library and development headers be discoverable
by [pkg_config](https://docs.rs/pkg-config/latest/pkg_config/). I have only
tried to build from Linux so other platforms are completely untested (as opposed
to mostly untested).

## Contribution

Contribution is always welcome. I recommend opening an issue before starting
work on anything, but at this point in the project I am very open to help.
