# squashed

This is an incomplete ergonomic wrapper
around [libsquashfs](https://infraroot.at/projects/squashfs-tools-ng/doxydoc/index.html).
It uses bindings generated by the `libsquashfs1-sys` crate in this repo. It is
very **early days** in the project. There are **no correctness checks**
besides manual interpretation of the apis and trying to get them correct.

## Usage

This library requires the shared library and development headers be discoverable
by [pkg_config](https://docs.rs/pkg-config/latest/pkg_config/). I have only
tried to build from Linux so other platforms are completely untested (as opposed
to mostly untested).

## Contributions

Contribution is always welcome. I recommend opening an issue before starting
work on anything, but at this point in the project I am very open to help.
