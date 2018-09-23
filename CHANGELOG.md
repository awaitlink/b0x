# Changelog
All notable changes to this project will be documented in this file.    
The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased] - ????-??-??
### Changed
- `✘` instead of `x`

## [0.3.1] - 2018-09-07
### Changed
- `➔` instead of `-->`, `x` instead of `--x`
- Indentation

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