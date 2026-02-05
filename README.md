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
use googletest_json_serde::json as j;
use serde_json::json;

let actual = json!({
    "vampire": { "name": "Nandor the Relentless", "age": 758, "familiar": "Guillermo" },
    "house": { "city": "Staten Island", "roommates": ["Laszlo", "Nadja", "Colin Robinson"] }
});

assert_that!(
    actual,
    j::pat!({
        "vampire": {
            "name": starts_with("Nandor"),
            "age": gt(500),
            "familiar": eq("Guillermo"),
        },
        "house": {
            "city": eq("Staten Island"),
            "roommates": j::unordered_elements_are![
                eq("Laszlo"),
                eq("Nadja"),
                contains_substring("Robinson"),
            ],
        },
        .. // allow extra fields
    })
);
```

## Features

- Object patterns:
  - `j::matches_pattern!` / `j::pat!` (strict or relaxed)
- Arrays:
  - Ordered: `j::elements_are!`
  - Unordered: `j::unordered_elements_are!`
  - Contains-each: `j::contains_each!`
  - Contained-in: `j::is_contained_in!`
  - Length: `j::len!`
  - Apply to all elements: `j::each!`
  - Type guard: `j::each_is_string()/number/boolean/null/array/object`
- Primitives and kinds:
  - `j::primitive!`, `j::is_number/integer/fractional_number/whole_number/string/boolean`, `j::is_true/false`, `j::is_null`, `j::is_not_null`, `j::is_empty_string/non_empty_string`, `j::is_empty_array/object`, `j::is_non_empty_array/object`
- Paths and shape:
  - `j::has_paths`, `j::has_only_paths`, `j::has_path_with!`
- Adapters (bridge to native matchers):
  - `j::as_string`, `j::as_bool`, `j::as_i64` (and other number types), `j::as_array`, `j::as_object`
- Optional fields:
  - `j::optional!`
- Clear diagnostics that point to the failing path or element.

## More Examples

### Primitives

```rust
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

assert_that!(json!(42),         j::primitive!(gt(40_i64)));
assert_that!(json!("Laszlo"),   j::primitive!(starts_with("Las")));
assert_that!(json!(true),       j::is_true());
assert_that!(json!(null),       j::is_null());
assert_that!(json!(7),          j::is_integer());
assert_that!(json!(7.0),        j::is_whole_number());
assert_that!(json!(7.25),       j::is_fractional_number());
```

### Adapters (Bridge to native matchers)

```rust
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

// Bridge to native googletest matchers with explicit type unwrapping
assert_that!(json!("123-ABC"), j::as_string(matches_regex(r"^\d{3}")));
assert_that!(json!(3.14159),   j::as_f64(near(3.14, 0.01)));
assert_that!(json!([1, 2, 3]), j::as_array(contains(j::as_i64(eq(2)))));
```

### Path value matching

```rust
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

let value = json!({"user": {"id": 7, "name": "Ada"}});
assert_that!(value, j::has_path_with!("user.name", "Ada"));
assert_that!(value, j::has_path_with!("user.id", json!(7)));
assert_that!(value, j::has_path_with!("user.name", starts_with("A")));
```

### Predicates

```rust
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

assert_that!(json!(42), j::predicate(|v| v.as_i64().map_or(false, |n| n > 0)));
assert_that!(json!("Energy vampire"), j::predicate(|v| v.as_str().map_or(false, |s| s.contains("Energy"))));
```

### Objects

```rust
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

assert_that!(
    json!({"name": "Laszlo", "age": 310, "familiar": null}),
    j::pat!({
        "name": starts_with("Las"),
        "age": gt(300),
        "familiar": j::is_null(),
        .. // allow extras like hobbies or cursed hats
    })
);
```

### Arrays

```rust
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

assert_that!(
    json!(["Nandor", 758, true]),
    j::elements_are![eq("Nandor"), j::is_number(), is_true()]
);

assert_that!(
    json!(["Laszlo", "Nadja", "Colin Robinson"]),
    j::unordered_elements_are![eq("Colin Robinson"), "Laszlo", "Nadja"]
);

assert_that!(
    json!(["familiar", 1, null]),
    j::contains_each![j::is_string(), j::is_not_null()]
);

assert_that!(
    json!(["Nandor", "Nadja"]),
    j::each_is_string()
);
```

### Combined Example

```rust
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

assert_that!(
    json!({
        "guests": [
            {"name": "Baron Afanas", "age": 2000},
            {"name": "The Guide", "age": 500}
        ],
        "house": { "city": "Staten Island", "roommates": 4 },
        "ignored": true
    }),
    j::pat!({
        "guests": j::unordered_elements_are![
            j::pat!({ "name": starts_with("Baron"), "age": gt(1500) }),
            j::pat!({ "name": eq("The Guide"), "age": ge(400) })
        ],
        "house": { "city": eq("Staten Island"), "roommates": eq(4) },
        ..
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
