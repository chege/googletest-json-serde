# GoogleTest Json Serde

A set of matcher macros for ergonomic JSON testing with [googletest-rust](https://docs.rs/googletest/).

These tiny, focused matchers make it effortless to assert on `serde_json::Value` in Rust tests.

## Features

- **Value**: Match JSON primitive values (`string`, `number` (i64/f64), `bool`) using the `json::value!` macro, and
  match `null` values with `json::is_null()`.
- **Object**: Pattern-match JSON objects by fields using the `json::matches_pattern!{...}` macro; supports both *
  *strict** mode (
  all fields must match and no extra fields) and **non-strict** mode (extra fields allowed via `..`).
- **Array**: Match arrays element-by-element (ordered, supports heterogeneous types) using the
  `json::elements_are![...]` macro.
- **Unordered array**: Match arrays where order does not matter using the `json::unordered_elements_are![...]` macro.
- **Contains-each (arrays)**: Require that each expected matcher matches a unique element in any order, allowing extra
  elements, with the `json::contains_each![...]` macro.
- **Containment (arrays)**: Assert that an array is a subset of the expected matchers (i.e., every actual element is
  accounted for) using the `json::is_contained_in![...]` macro.
- Clear, structured **diagnostic failure messages**: When a match fails, detailed explanations show which part of the
  JSON structure did not match and why.

## Getting Started

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
googletest = "0.14"
serde_json = "1"
googletest-json-serde = "0.1" # replace with the latest version on crates.io
```

This crate is typically only needed as a dev-dependency.

In tests:

```rust
use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;
```

> The crate re-exports a `json` namespace with everything you need:
> - `json::value!(...)` – match a value inside a JSON `Value`
> - `json::matches_pattern!` – explicitly match JSON objects by fields (with `json::pat!` as an alias)
> - `json::elements_are![...]` – match arrays element-by-element (ordered)
> - `json::unordered_elements_are![...]` – match arrays element-by-element (unordered)
> - `json::is_contained_in![...]` – assert containment of JSON elements

---

## Examples

### Values

#### Match JSON values with `json::value`

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

fn values() {
    assert_that!(j!(42),        json::value!(gt(40i64)));
    assert_that!(j!(3.14),      json::value!(near(3.1f64, 0.1f64)));
    assert_that!(j!("hello"),   json::value!(starts_with("he")));
    assert_that!(j!(true),      json::value!(is_true()));
    assert_that!(j!(null),     json::is_null());
}
```

---

### Objects

#### Match JSON objects with `json::pat!`

Strict match:

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

fn object_strict() {
    let v = j!({"name": "Alice", "age": 30.0});
    assert_that!(
        v,
        json::pat!({
            "name": eq("Alice"),
            "age":  eq(30.0),
        })
    );
}
```

Non-strict match (with `..`):

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

fn object_non_strict() {
    let v = j!({"name": "Alice", "age": 30.0, "extra": "ignored"});
    assert_that!(
        v,
        json::pat!({
            "name": eq("Alice"),
            ..
        })
    );
}
```

---

### Arrays

#### Match arrays with `json::elements_are!` (ordered)

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

fn arrays_ordered() {
    assert_that!(
        j!(["hello", 42, true]),
        json::elements_are![eq("hello"), eq(42), eq(true)]
    );
}
```

#### Match arrays with `json::unordered_elements_are!` (unordered)

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

fn arrays_unordered() {
    assert_that!(
        j!([42, "hello", true]),
        json::unordered_elements_are![eq("hello"), eq(42), eq(true)]
    );
}
```

#### Assert containment with `json::is_contained_in!`

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

fn containment() {
    assert_that!(
        j!(["a", "b", "c"]),
        json::is_contained_in![eq("a"), eq("c")]
    );
}
```

---

### Combined example

Compose all together for complex structures. Here is a Rust struct with a JSON field, using `matches_pattern!` for the
struct and nested JSON matchers for the field:

```rust
use googletest::prelude::*;
use googletest::matchers::matches_pattern;
use googletest_json_serde::json;
use serde_json::json as j;

#[derive(Debug)]
struct Response {
    status: u32,
    payload: serde_json::Value,
}

fn combined_match() {
    let resp = Response {
        status: 200,
        payload: j!({
            "user": {
                "name": "Ali",
                "tags": ["admin", "tester"],
                "active": true,
                "ignored": false,
            }
        }),
    };

    assert_that!(
        resp,
        matches_pattern!(Response {
            status: eq(&200),
            payload: json::pat!({
                "user": json::pat!({
                    "name": json::value!(starts_with("Ali")),
                    "tags": json::elements_are![eq("admin"), eq("tester")],
                    "active": json::value!(is_true()),
                    ..
                })
            })
        })
    );
}
```

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

Files that are copied or adapted from [googletest-rust](https://github.com/google/googletest-rust) retain their original
Apache 2.0 license header with a notice of modifications.