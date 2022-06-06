# Changelog
All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2022-06-06
### Changed
- Upgraded dependencies to fix security issues.

## [1.0.0] - 2019-05-11
### Changed
- Release 1.0.0

## [0.8.0] - 2019-03-29
### Changed
- Moved number of `zeros - leading zeros` from `()` in `integer -> radix -> zeros` to `integer -> radix -> visible zeros`.
- Implementation design of passes.

## [0.7.2] - 2018-12-31
### Changed
- Use `term_size` instead of `termion`.
- Color tweaks.

## [0.7.1] - 2018-12-31
### Changed
- Merged `ip_addr -> {general, specific}` into `ip_addr -> information`.
- Merged `ip_addr -> {general, specific} -> {unspecified?, loopback?, multicast?, private?, link-local?, broadcast?, documentation?}` into `ip_addr -> information -> kind` since at most one of these properties should be `true` for a given IP address.
- Renamed `ip_addr -> conversions -> {ipv6-compatible, ipv6-mapped}` to `ip_addr -> conversions -> {ipv6 compatible, ipv6 mapped}`.

## [0.7.0] - 2018-12-30
### Added
- Parsing of and information for IP addresses (via `IpAddr`), see `README.md` for more information!
- You can now specify multiple inputs!
- Pass `integer -> modifications`.
- Information `integer -> modifications -> swap bytes`.
- Information `string -> structure -> empty?`.

### Changed
- Ignoring passes is now easier! Just specify first characters of the passes you want to ignore; see `b0x --help` for more information.
- Due to the change above, `prime` pass was renamed to `is_prime` in order to avoid two passes for same data type starting with the same character (`prime` and `power`).

### Fixed
- Now input can be placed after options.

## [0.6.0] - 2018-12-07
### Changed
- Rust 2018 :tada:

## [0.5.0] - 2018-10-19
### Added
- Pass for converting integers to english words.

## [0.4.0] - 2018-10-14
### Added
- Prime and perfect power checks.

## [0.3.2] - 2018-10-14
### Changed
- `✘` instead of `x`.
- `•` instead of `⋅`.

## [0.3.1] - 2018-09-07
### Changed
- `➔` instead of `-->`, `x` instead of `--x`.
- Indentation.

## [0.3.0] - 2018-09-01
### Changed
- A lot more information about strings, including grapheme clusters and words.
- String processing split into multiple passes.
- `-->` instead of `pass`, and `--x` when the pass is ignored.

## [0.2.1] - 2018-08-31
### Changed
- Fix typos in documentation.

## [0.2.0] - 2018-08-31
### Added
- Now `bin -> zeros` displays how many zeros are actually visible, in addition to total count.
- Ability to ignore specified passes. For example:
  ```console
  $ b0x 0xC0FFEE --ignore misc
  ```

  See

  ```console
  $ b0x --help
  ```

  for more information.

### Changed
- Output separated by passes.

## [0.1.1] - 2018-08-30
### Changed
- Absolute image links in README.md.

## [0.1.0] - 2018-08-30
### First release

[1.0.1]: https://github.com/u32i64/b0x/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/u32i64/b0x/compare/v0.8.0...v1.0.0
[0.8.0]: https://github.com/u32i64/b0x/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/u32i64/b0x/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/u32i64/b0x/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/u32i64/b0x/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/u32i64/b0x/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/u32i64/b0x/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/u32i64/b0x/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/u32i64/b0x/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/u32i64/b0x/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/u32i64/b0x/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/u32i64/b0x/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/u32i64/b0x/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/u32i64/b0x/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/u32i64/b0x/releases/tag/v0.1.0
