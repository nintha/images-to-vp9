# vpx-encode

Rust interface to libvpx encoder

This crate provides a Rust API to use
[libvpx](https://en.wikipedia.org/wiki/Libvpx) for encoding images.

It it based entirely on code from [srs](https://crates.io/crates/srs).
Compared to the original `srs`, this code has been simplified for use as a
library and updated to add support for both the VP8 codec and (optionally)
the VP9 codec.

## Optional features

Compile with the cargo feature `vp9` to enable support for the VP9 codec.

## Example

An example of using `vpx-encode` can be found in the [`record-screen`]()
program. The source code for `record-screen` is in the [vpx-encode git
repository]().

## Contributing

All contributions are appreciated.

License: MIT
