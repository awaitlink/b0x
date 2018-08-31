# ❮b0x❯

[![version][badges/version]][crates.io/b0x]
[![downloads][badges/downloads]][crates.io/b0x]
[![license][badges/license]][license]

A simple CLI tool to display information about the provided input.

### Installation
```console
$ cargo install b0x
```

### Usage
#### Numbers
Supported formats:
- Decimal: `42`
- Binary: `0b101010`
- Octal: `0o52`
- Hexadecimal: `0x2A`

```console
$ b0x 0xC0FFEE
```

![screenshot][demo-0xC0FFEE]

#### Strings

```console
$ b0x "TeSt StRiNg"
```

![screenshot][demo-string]

[crates.io/b0x]: https://crates.io/crates/b0x

[license]: https://github.com/u32i64/b0x/blob/master/LICENSE
[changelog]: https://github.com/u32i64/b0x/blob/master/CHANGELOG.md

[badges/version]: https://img.shields.io/crates/v/b0x.svg?style=for-the-badge
[badges/downloads]: https://img.shields.io/crates/d/b0x.svg?style=for-the-badge
[badges/license]: https://img.shields.io/crates/l/b0x.svg?style=for-the-badge

[demo-0xC0FFEE]: https://raw.githubusercontent.com/u32i64/b0x/master/img/0xc0ffee.png
[demo-string]: https://raw.githubusercontent.com/u32i64/b0x/master/img/string.png