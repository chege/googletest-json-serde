![Logo](https://raw.githubusercontent.com/chege/googletest-json-serde/main/assets/logo.svg)

<div align="center">
  <h3>
    <a href="#overview">Overview</a>
    <span> | </span>
    <a href="#installation">Installation</a>
    <span> | </span>
    <a href="#usage">Usage</a>
    <span> | </span>
    <a href="#features">Features</a>
    <span> | </span>
    <a href="#more-examples">More Examples</a>
    <span> | </span>
    <a href="#documentation">Documentation</a>
    <span> | </span>
    <a href="#contributing">Contributing</a>
    <span> | </span>
    <a href="#license">License</a>
  </h3>
</div>

<p align="center"><em>
A set of matcher macros for ergonomic JSON testing with <a href="https://docs.rs/googletest/">googletest-rust</a>.<br><br>
These tiny, focused matchers make it effortless to assert on <code>serde_json::Value</code> in Rust tests.
</em></p>

<p align="center">
  <a href="https://crates.io/crates/googletest-json-serde"><img src="https://img.shields.io/crates/v/googletest-json-serde.svg" alt="Crates.io"></a>
  <a href="https://github.com/chege/googletest-json-serde/actions/workflows/ci.yaml"><img src="https://github.com/chege/googletest-json-serde/actions/workflows/ci.yaml/badge.svg" alt="CI"></a>
  <a href="https://docs.rs/googletest-json-serde"><img src="https://img.shields.io/docsrs/googletest-json-serde" alt="Docs.rs"></a>
  <a href="https://github.com/chege/googletest-json-serde/blob/main/LICENSE-MIT"><img src="https://img.shields.io/crates/l/googletest-json-serde" alt="License"></a>
</p>

## Overview

googletest-json-serde adds focused matcher macros for JSON so your Rust tests read like intent, not plumbing. It handles
heterogeneous arrays, deep object patterns, path checks, and produces readable failure messages with path context.

## Installation

Add as a dev-dependency:

```bash
cargo add googletest-json-serde --dev
```

## Usage

```rust
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

let actual = j!({
    "vampire": { "name": "Nandor the Relentless", "age": 758, "familiar": "Guillermo" },
    "house": { "city": "Staten Island", "roommates": ["Laszlo", "Nadja", "Colin Robinson"] }
});

assert_that!(
    actual,
    json::pat!({
        "vampire": json::pat!({
            "name": starts_with("Nandor"),
            "age": gt(500),
            "familiar": eq("Guillermo"),
        }),
        "house": json::pat!({
            "city": eq("Staten Island"),
            "roommates": json::unordered_elements_are![
                eq("Laszlo"),
                eq("Nadja"),
                contains_substring("Robinson"),
            ],
        }),
        .. // allow extra fields
    })
);
```

## Features

- Object patterns:
  - `json::matches_pattern!` / `json::pat!` (strict or relaxed)
- Arrays:
  - Ordered: `json::elements_are!`
  - Unordered: `json::unordered_elements_are!`
  - Contains-each: `json::contains_each!`
  - Contained-in: `json::is_contained_in!`
  - Length: `json::len!`
  - Apply to all elements: `json::each!`
  - Type guard: `json::each_is_string()/number/boolean/null/array/object`
- Primitives and kinds:
  - `json::primitive!`, `json::is_number/integer/fractional_number/whole_number/string/boolean`, `json::is_true/false`, `json::is_null`, `json::is_not_null`, `json::is_empty_string/non_empty_string`, `json::is_empty_array/object`, `json::is_non_empty_array/object`
- Paths and shape:
  - `json::has_paths`, `json::has_only_paths`, `json::has_path_with!`
- Optional fields:
  - `json::optional!`
- Clear diagnostics that point to the failing path or element.

## More Examples

### Primitives

```rust
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

assert_that!(j!(42),         json::primitive!(gt(40_i64)));
assert_that!(j!("Laszlo"),   json::primitive!(starts_with("Las")));
assert_that!(j!(true),       json::is_true());
assert_that!(j!(null),       json::is_null());
assert_that!(j!(7),          json::is_integer());
assert_that!(j!(7.0),        json::is_whole_number());
assert_that!(j!(7.25),       json::is_fractional_number());
```

### Path value matching

```rust
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

let value = j!({"user": {"id": 7, "name": "Ada"}});
assert_that!(value, json::has_path_with!("user.name", "Ada"));
assert_that!(value, json::has_path_with!("user.id", j!(7)));
assert_that!(value, json::has_path_with!("user.name", starts_with("A")));
```

### Predicates

```rust
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

assert_that!(j!(42), json::predicate(|v| v.as_i64().map_or(false, |n| n > 0)));
assert_that!(j!("Energy vampire"), json::predicate(|v| v.as_str().map_or(false, |s| s.contains("Energy"))));
```

### Objects

```rust
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

assert_that!(
    j!({"name": "Laszlo", "age": 310, "familiar": null}),
    json::pat!({
        "name": starts_with("Las"),
        "age": gt(300),
        "familiar": json::is_null(),
        .. // allow extras like hobbies or cursed hats
    })
);
```

### Arrays

```rust
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

assert_that!(
    j!(["Nandor", 758, true]),
    json::elements_are![eq("Nandor"), json::is_number(), is_true()]
);

assert_that!(
    j!(["Laszlo", "Nadja", "Colin Robinson"]),
    json::unordered_elements_are![eq("Colin Robinson"), "Laszlo", "Nadja"]
);

assert_that!(
    j!(["familiar", 1, null]),
    json::contains_each![json::is_string(), json::is_not_null()]
);

assert_that!(
    j!(["Nandor", "Nadja"]),
    json::each_is_string()
);
```

### Combined Example

```rust
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

assert_that!(
    j!({
        "guests": [
            {"name": "Baron Afanas", "age": 2000},
            {"name": "The Guide", "age": 500}
        ],
        "house": { "city": "Staten Island", "roommates": 4 }
    }),
    json::pat!({
        "guests": json::unordered_elements_are![
            json::pat!({ "name": starts_with("Baron"), "age": gt(1500) }),
            json::pat!({ "name": eq("The Guide"), "age": ge(400) })
        ],
        "house": json::pat!({ "city": eq("Staten Island"), "roommates": eq(4) }),
    })
);
```

## Documentation

- API reference: <https://docs.rs/googletest-json-serde>
- Crate: <https://crates.io/crates/googletest-json-serde>
- More usage patterns live in `tests/` and `sanity/tests/sanity_test.rs`.

## Contributing

- Issues: <https://github.com/chege/googletest-json-serde/issues>
- Contributions welcome!
  See [CONTRIBUTING.md](https://github.com/chege/googletest-json-serde/blob/main/CONTRIBUTING.md).

## License

Dual-licensed under MIT or Apache-2.0.
