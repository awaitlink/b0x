# ❮b0x❯

[![version][badges/version]][crates.io/b0x]
[![downloads][badges/downloads]][crates.io/b0x]
[![license][badges/license]][license]

A simple CLI tool to display information about the provided input (integers, IP addresses, strings).

### Installation
```console
$ cargo install b0x
```

### Updating
```console
$ cargo install b0x --force
```

### Usage
```console
$ b0x <input>...
```

Supported data types:
- IP addresses
    - IPv4: `1.1.1.1`
    - IPv6: `2606:4700:4700::1111`
- Unsigned integers
    - Binary: `0b101010`
    - Octal: `0o52`
    - Hexadecimal: `0x2A`
    - Decimal: `42`
- Strings

See the built-in help for more information:
```console
$ b0x --help
```

### Examples

```console
$ b0x 127.0.0.1 2606:4700:4700::1111
```

![screenshot/ip]

```console
$ b0x 0xC0FFEE
```

![screenshot/int]

```console
$ b0x "TeSt StRiNg"
```

![screenshot/str]

[crates.io/b0x]: https://crates.io/crates/b0x

[license]: https://github.com/u32i64/b0x/blob/master/LICENSE
[changelog]: https://github.com/u32i64/b0x/blob/master/CHANGELOG.md

[badges/version]: https://img.shields.io/crates/v/b0x.svg?style=for-the-badge
[badges/downloads]: https://img.shields.io/crates/d/b0x.svg?style=for-the-badge
[badges/license]: https://img.shields.io/crates/l/b0x.svg?style=for-the-badge

[screenshot/ip]: https://raw.githubusercontent.com/u32i64/b0x/master/img/ip.png
[screenshot/int]: https://raw.githubusercontent.com/u32i64/b0x/master/img/int.png
[screenshot/str]: https://raw.githubusercontent.com/u32i64/b0x/master/img/str.png
