# googletest-serde-json

Tiny, focused matchers and macros that make it effortless to assert on `serde_json::Value` in Rust tests
using [googletest-rust](https://docs.rs/googletest/).

- ✅ **Scalar** assertions for JSON strings, numbers (i64/f64), and booleans
- ✅ **Object** pattern matching with strict / non-strict modes
- ✅ **Array** element-by-element matching (supports heterogenous types)
- ✅ Clear, structured **failure explanations**

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
googletest = "0.14"
serde_json = "1"

# this crate
googletest-serde-json = { path = "." } # or use the registry name/version when published
```

In tests:

```rust
use googletest::prelude::*;
use googletest_serde_json::json;
use serde_json::json as j;
```

> The crate re-exports a `json` namespace with everything you need:
>
> - `json::scalar(...)` – match a scalar inside a JSON `Value`
> - `json::pat!{...}` – match a JSON object by fields
> - `json::elements_are!(...)` – match arrays element-by-element

---

## Quick start

### Match JSON scalars with `json::scalar`

```rust
#[test]
fn scalars() {
    assert_that!(j!(42),       json::scalar(gt(40)));           // i64
    assert_that!(j!(3.14),     json::scalar(close_to(3.1, 0.1))); // f64
    assert_that!(j!("hello"),  json::scalar(starts_with("he"))); // &str
    assert_that!(j!(true),     json::scalar(is_true()));         // bool
}
```

---

### Match JSON objects with `json::pat!`

Strict match:

```rust
#[test]
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
#[test]
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

### Match arrays with `json::elements_are!`

```rust
#[test]
fn arrays() {
    assert_that!(
        j!(["hello", 42, true]),
        json::elements_are!(eq("hello"), eq(42), eq(true))
    );
}
```

---

## Combined example

You can compose all three together for complex structures.  
Here we have a Rust struct with a JSON field, using `matches_pattern!` for the struct and nested JSON matchers for the
field:

```rust
use googletest::prelude::*;
use googletest::matchers::matches_pattern;
use googletest_serde_json::json;
use serde_json::json as j;

#[derive(Debug)]
struct Response {
    status: u32,
    payload: serde_json::Value,
}

#[test]
fn combined_match() {
    let resp = Response {
        status: 200,
        payload: j!({
            "user": {
                "name": "Alice",
                "tags": ["admin", "tester"],
                "active": true,
                "extra": "ignored"
            }
        }),
    };

    assert_that!(
        resp,
        matches_pattern!(Response {
            status: eq(200),
            payload: json::pat!({
                "user": json::pat!({
                    "name": json::scalar(starts_with("Ali")),
                    "tags": json::elements_are!(eq("admin"), eq("tester")),
                    "active": json::scalar(is_true()),
                    ..
                })
            })
        })
    );
}
```

---