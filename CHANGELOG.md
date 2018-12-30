# Changelog
All notable changes to this project will be documented in this file.    
The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2018-12-30
### Added
- Parsing of and information for IP addresses (via `IpAddr`), see `README.md` for more information!
- You can now specify multiple inputs!
- Pass `integer -> modifications`
- Information `integer -> modifications -> swap bytes`
- Information `string -> structure -> empty?`

### Changed
- Ignoring passes is now easier! Just specify first characters of the passes you want to ignore; see `b0x --help` for more information
- Due to the change above, `prime` pass was renamed to `is_prime` in order to avoid two passes for same data type starting with the same character (`prime` and `power`)

### Fixed
- Now input can be placed after options

## [0.6.0] - 2018-12-07
### Changed
- Rust 2018 :tada:

## [0.5.0] - 2018-10-19
### Added
- Pass for converting integers to english words

## [0.4.0] - 2018-10-14
### Added
- Prime and perfect power checks

## [0.3.2] - 2018-10-14
### Changed
- `✘` instead of `x`
- `•` instead of `⋅`

## [0.3.1] - 2018-09-07
### Changed
- `➔` instead of `-->`, `x` instead of `--x`
- Indentation

## [0.3.0] - 2018-09-01
### Changed
- A lot more information about strings, including grapheme clusters and words
- String processing split into multiple passes
- `-->` instead of `pass`, and `--x` when the pass is ignored

## [0.2.1] - 2018-08-31
### Changed
- Fix typos in documentation

## [0.2.0] - 2018-08-31
### Added
- Now `bin -> zeros` displays how many zeros are actually visible, in addition to total count
- Ability to ignore specified passes. For example:
  ```console
  $ b0x 0xC0FFEE --ignore misc
  ```
  
  See

  ```console
  $ b0x --help
  ```
  
  for more information

### Changed
- Output separated by passes

## [0.1.1] - 2018-08-30
### Changed
- Absolute image links in README.md

## [0.1.0] - 2018-08-30
### First release
