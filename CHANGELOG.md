# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.2] - 2020-12-06

### Fixed

- Fix warning `clippy::self_assignment`.

## [1.0.1] - 2020-09-12

### Fixed

- Fix warning `unused_attributes`. `#![warn(rust_2018_idioms)]`s that were not
  specified at crate level were just useless.

## [1.0.0] - 2020-07-26

[Unreleased]: https://github.com/dkim/i8080/compare/1.0.2...HEAD
[1.0.2]: https://github.com/dkim/i8080/compare/1.0.1...1.0.2
[1.0.1]: https://github.com/dkim/i8080/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/dkim/i8080/releases/tag/1.0.0
