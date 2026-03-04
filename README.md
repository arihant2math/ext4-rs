# ext4plus

This repository provides a Rust crate that allows access and modification to an
[ext4] filesystem. It also works with [ext2] and [ext3] filesystems.
The crate is `no_std`, so it can be used in embedded or osdev contexts. However, it does require `alloc`.

[ext2]: https://en.wikipedia.org/wiki/Ext2
[ext3]: https://en.wikipedia.org/wiki/Ext3
[ext4]: https://en.wikipedia.org/wiki/Ext4

This project is a hard fork of [ext4-view-rs](https://github.com/nicholasbishop/ext4-view-rs/),
which is a read-only ext4 library.
The goal of this fork is to add write and async support, which requires some significant changes to the API and internal design.

Additionally, due to the need for more low-level access to the filesystem, this crate exposes a greater API surface.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.
