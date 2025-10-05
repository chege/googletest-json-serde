<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/logo-dark.svg" >
  <source media="(prefers-color-scheme: light)" srcset="assets/logo-light.svg" >
  <img alt="GoogleTest JSON Serde logo" src="assets/logo-dark.svg">
</picture>
<div align="center">
  <h3>
    <a href="#getting-started">Getting Started</a>
    <span> | </span>
    <a href="#examples">Examples</a>
    <span> | </span>
    <a href="#values">Values</a>
    <span> | </span>
    <a href="#objects">Objects</a>
    <span> | </span>
    <a href="#arrays">Arrays</a>
    <span> | </span>
    <a href="#combined-example">Combined example</a>
  </h3>
</div>


A set of matcher macros for ergonomic JSON testing with [googletest-rust](https://docs.rs/googletest/).

These tiny, focused matchers make it effortless to assert on `serde_json::Value` in Rust tests.

## Features

- **Value**: Match JSON primitive values (`string`, `number` (i64/f64), `bool`) using the `json::primitive!` macro, and
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
> - `json::primitive!(...)` – match a value inside a JSON `Value`
> - `json::matches_pattern!` – explicitly match JSON objects by fields (with `json::pat!` as an alias)
> - `json::elements_are![...]` – match arrays element-by-element (ordered)
> - `json::unordered_elements_are![...]` – match arrays element-by-element (unordered)
> - `json::is_contained_in![...]` – assert containment of JSON elements

## Examples

### Primitives

#### Match JSON primitives with `json::primitive`

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

assert_that!(j!(42),        json::primitive!(gt(40i64)));
assert_that!(j!(3.14),      json::primitive!(near(3.1f64, 0.1f64)));
assert_that!(j!("hello"),   json::primitive!(starts_with("he")));
assert_that!(j!(true),      json::primitive!(is_true()));
assert_that!(j!(null),     json::is_null());
```

### Objects

#### Match JSON objects with `json::matches_pattern!`

Strict match:

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

let v = j!({"name": "Alice", "age": 30.0});
assert_that!(
    v,
    json::matches_pattern!({
        "name": eq("Alice"),
        "age":  ge(30.0),
    })
);
```

Non-strict match (with `..`):

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

let v = j!({"name": "Alice", "age": 30.0, "extra": "ignored"});
assert_that!(
    v,
    json::matches_pattern!({
        "name": eq("Alice"),
        ..
    })
);
```

### Arrays

#### Match arrays with `json::elements_are!` (ordered)

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

assert_that!(
    j!(["hello", 42, true]),
    json::elements_are![eq("hello"), eq(42), is_true()]
);
assert_that!(
    j!(["hello", 42, true]),
    not(json::elements_are![eq("hello"), eq(42), is_false()])
);
```

#### Match arrays with `json::unordered_elements_are!` (unordered)

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

assert_that!(
    j!([42, "hello", true]),
    json::unordered_elements_are![eq(42), eq(true), eq("hello")]
);
assert_that!(
    j!([42, "hello", true]),
    not(json::unordered_elements_are![eq(42), eq(false), eq("hello")])
);
```

#### Assert containment with `json::is_contained_in!`

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

assert_that!(
    j!(["a", "b", "c"]),
    json::is_contained_in![eq("a"), starts_with("b"), eq("c"), eq("d")]
);
assert_that!(
    j!(["a", "b", "c"]),
    not(json::is_contained_in![eq("a"), eq("c")])
);
```

#### Assert each matcher finds a unique element with json::contains_each!

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

// Array can have extra elements, but must contain all required ones
assert_that!(
    j!(["admin", "user", "tester", "viewer"]),
    json::contains_each![eq("admin"), starts_with("test")]
);
assert_that!(
    j!(["admin", "user", "tester", "viewer"]),
    not(json::contains_each![eq("admin"), eq("missing")])
);
```

#### Array Matcher Quick Reference

Here’s a quick reference matrix comparing the array matchers:

| Matcher                   | Order Matters | Extra Elements OK | Missing Elements OK | Use Case                                                      |
|---------------------------|---------------|-------------------|---------------------|---------------------------------------------------------------|
| `elements_are!`           | Yes           | No                | No                  | Exact ordered match of all elements                           |
| `unordered_elements_are!` | No            | No                | No                  | Exact unordered match of all elements                         |
| `contains_each!`          | No            | Yes               | No                  | Require each matcher to match a unique element, extra allowed |
| `is_contained_in!`        | No            | No                | Yes                 | Actual elements are subset of expected                        |

> 💡 **Note:** All JSON matcher macros support both direct matchers (e.g. `starts_with("x")`) and explicit
> `json::primitive!(...)` wrappers. Use whichever makes intent clearer.

### Combined example

Compose all together for complex structures. Here is a Rust struct with a JSON field, using `matches_pattern!` for the
struct and nested JSON matchers for the field:

```rust
# use googletest::prelude::*;
# use googletest_json_serde::json;
# use serde_json::json as j;

#[derive(Debug)]
struct Response {
    status: u32,
    payload: serde_json::Value,
}

let resp = Response {
    status: 200,
    payload: j!({
        "user": {
            "id": 42,
            "name": "Jeff",
            "roles": ["admin", "user", "tester"],
            "tags": ["rust", "serde", "dev"],
            "scores": [99, 87, 75],
            "matrix": [
                ["alpha", 1],
                ["beta", 2, "extra"]
            ],
            "settings": {
                "theme": "dark",
                "email": true,
                "beta": null
            },
            "metadata": {
                "created": "2025-10-05"
            }
        },
        "extra": { "debug": true }
    }),
};

assert_that!(
    resp,
    matches_pattern!(Response {
        status: eq(&200),
        payload: json::matches_pattern!({
            "user": json::matches_pattern!({
                // native matchers inside objects
                "id": gt(0),
                "name": starts_with("Je"),

                // ordered array of native matchers
                "roles": json::elements_are![
                    starts_with("adm"),
                    eq("user"),
                    ends_with("er")
                ],

                // unordered array, native matchers directly
                "tags": json::unordered_elements_are![
                    starts_with("se"),
                    eq("dev"),
                    ends_with("st")
                ],

                // array of arrays — demonstrate nesting twice
                "matrix": json::elements_are![
                    json::is_contained_in![starts_with("al"), json::any_value()],
                    json::is_contained_in![starts_with("be"), json::any_value(), json::any_value()]
                ],

                // object with mixed matchers
                "settings": json::matches_pattern!({
                    "theme": eq("dark"),
                    "email": is_true(),
                    "beta": json::is_null(),
                }),

                // remaining fields ignored
                ..
            }),
            ..
        })
    })
);

```


## Acknowledgements

Parts of this crate are adapted from [googletest-rust](https://github.com/google/googletest-rust), which is licensed
under Apache 2.0.
