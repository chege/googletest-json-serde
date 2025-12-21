# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.4.6](https://github.com/chege/googletest-json-serde/compare/v0.4.5...v0.4.6) - 2025-12-21

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.4.5/googletest-json-serde/0.4.6/Cargo.toml)

### <!-- 0 -->⛰️ Features

- add string emptiness matchers
- add type-specific each_is_* matchers
- add is_non_empty_object and is_non_empty_array matchers

## [0.4.5](https://github.com/chege/googletest-json-serde/compare/v0.4.4...v0.4.5) - 2025-12-03

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.4.4/googletest-json-serde/0.4.5/Cargo.toml)

### <!-- 0 -->⛰️ Features

- add is_fractional_number matcher
- add is_whole_number matcher
- add is_integer matcher

## [0.4.4](https://github.com/chege/googletest-json-serde/compare/v0.4.3...v0.4.4) - 2025-11-30

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.4.3/googletest-json-serde/0.4.4/Cargo.toml)

### <!-- 0 -->⛰️ Features

- add is_true/is_false matchers
- add has_path_with matcher

### <!-- 1 -->🐛 Bug Fixes

- *(docs)* rewrite README and matcher docs, add examples, errors sections, and clean structure

### <!-- 3 -->📚 Documentation

- tidy matcher docs and normalize notes
- hide internal matchers and expose public names at crate root

## [0.4.3](https://github.com/chege/googletest-json-serde/compare/v0.4.2...v0.4.3) - 2025-11-23

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.4.2/googletest-json-serde/0.4.3/Cargo.toml)

### <!-- 0 -->⛰️ Features

- add is_empty_object, has_paths, and has_only_paths matchers

### <!-- 7 -->⚙️ Miscellaneous Tasks

- remove unused dependency `serde`

## [0.4.2](https://github.com/chege/googletest-json-serde/compare/v0.4.1...v0.4.2) - 2025-11-18

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.4.1/googletest-json-serde/0.4.2/Cargo.toml)

### <!-- 0 -->⛰️ Features

- add array matcher `json::each`
- add array matcher `json::len`

### <!-- 7 -->⚙️ Miscellaneous Tasks

- exclude sanity folder from published crate

## [0.4.1](https://github.com/chege/googletest-json-serde/compare/v0.4.0...v0.4.1) - 2025-11-11

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.4.0/googletest-json-serde/0.4.1/Cargo.toml)

### <!-- 0 -->⛰️ Features

- add optional matcher

### <!-- 2 -->🚜 Refactor

- migrate all matchers to unified JsonMatcher trait

## [0.4.0](https://github.com/chege/googletest-json-serde/compare/v0.3.1...v0.4.0) - 2025-11-04

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.3.1/googletest-json-serde/0.4.0/Cargo.toml)

### <!-- 0 -->⛰️ Features

- [**breaking**] add support for primitive literal matching in json::pat!

## [0.3.1](https://github.com/chege/googletest-json-serde/compare/v0.3.0...v0.3.1) - 2025-11-02

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.3.0/googletest-json-serde/0.3.1/Cargo.toml)

### <!-- 0 -->⛰️ Features

- *(json)* add support for all integer types (i8, i16, u8, u16, u32)

## [0.3.0](https://github.com/chege/googletest-json-serde/compare/v0.2.4...v0.3.0) - 2025-10-26

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.2.4/googletest-json-serde/0.3.0/Cargo.toml)

### <!-- 0 -->⛰️ Features

- [**breaking**] add support for direct serde_json::Value equality in JSON matchers

## [0.2.4](https://github.com/chege/googletest-json-serde/compare/v0.2.3...v0.2.4) - 2025-10-22

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.2.3/googletest-json-serde/0.2.4/Cargo.toml)

### <!-- 0 -->⛰️ Features

- add `is_empty_array()` matcher

### <!-- 3 -->📚 Documentation

- *(readme)* add grey logo

## [0.2.3](https://github.com/chege/googletest-json-serde/compare/v0.2.2...v0.2.3) - 2025-10-15

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.2.2/googletest-json-serde/0.2.3/Cargo.toml)

### <!-- 0 -->⛰️ Features

- add predicate matcher function

### <!-- 7 -->⚙️ Miscellaneous Tasks

- exclude non-essential files from published crate

## [0.2.2](https://github.com/chege/googletest-json-serde/compare/v0.2.1...v0.2.2) - 2025-10-15

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.2.1/googletest-json-serde/0.2.2/Cargo.toml)

### <!-- 0 -->⛰️ Features

- *(json)* add `is_not_null` matcher; deprecate `any_value`

### <!-- 3 -->📚 Documentation

- fix img url

### <!-- 7 -->⚙️ Miscellaneous Tasks

- taskfile

## [0.2.1](https://github.com/chege/googletest-json-serde/compare/v0.2.0...v0.2.1) - 2025-10-07

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.2.0/googletest-json-serde/0.2.1/Cargo.toml)

### <!-- 0 -->⛰️ Features

- *(matchers)* add JsonPredicateMatcher and type-specific JSON matchers

### <!-- 1 -->🐛 Bug Fixes

- *(assets)* correct malformed SVG by removing stray closing tag in mask section

### <!-- 2 -->🚜 Refactor

- *(matchers)* split value_matcher into json_matcher and primitive_matcher

### <!-- 3 -->📚 Documentation

- *(readme)* simplify badge layout and center project description
- improve README.md

### <!-- 7 -->⚙️ Miscellaneous Tasks

- *(ci)* optimize caching and build settings for faster pipeline execution
- simplify bug report
- add weekly sanity check workflow and crate validation script

## [0.2.0](https://github.com/chege/googletest-json-serde/compare/v0.1.0...v0.2.0) - 2025-10-06

[View diff on diff.rs](https://diff.rs/googletest-json-serde/0.1.0/googletest-json-serde/0.2.0/Cargo.toml)

### <!-- 0 -->⛰️ Features

- [**breaking**] support native matchers in array and object macros; add any_value for non-null match
- add `json::primitive!` macro and deprecate `json::value!`
- add structured GitHub issue templates for bugs and feature requests

### <!-- 2 -->🚜 Refactor

- move __json_is_contained_in macro from elements_are_matcher

### <!-- 3 -->📚 Documentation

- *(readme)* replace deprecated json::value with json::primitive and update section title

### <!-- 6 -->🧪 Testing

- add nested and edge case coverage for JSON matchers

### <!-- 7 -->⚙️ Miscellaneous Tasks

- add automated release workflow using Release-plz

## [0.1.0](https://github.com/chege/googletest-json-serde/releases/tag/v0.1.0) - 2025-09-27



### <!-- 3 -->📚 Documentation

- *(readme)* update with new examples, array matcher reference, and logo assets

### <!-- 6 -->🧪 Testing

- expand coverage and improve error checks
- expand coverage for array matchers and improve error checks

### <!-- 7 -->⚙️ Miscellaneous Tasks

- create CODE_OF_CONDUCT.md
- add GitHub workflow for daily dependency audit using cargo-deny
- add release-plz.toml with changelog config
- add CODEOWNERS
- improve workflow with toolchain setup, msrv, docs, and windows tests
- initial commit

### Build

- set up commitlint and husky for conventional commits
