# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


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
