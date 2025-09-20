# googletest-serde-json

Tiny, focused matchers and macros that make it effortless to assert on `serde_json::Value` in Rust tests using [googletest-rust](https://docs.rs/googletest/).

- ✅ **Value** assertions for JSON strings, numbers (i64/f64), booleans, and null  
- ✅ **Object** pattern matching with strict / non-strict modes  
- ✅ **Array** element-by-element matching (supports heterogeneous types)  
- ✅ **Unordered array** matching  
- ✅ **Containment** assertions (check if JSON contains a subset)  
- ✅ Clear, structured **failure explanations**

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
googletest = "0.14"
serde_json = "1"
googletest-serde-json = "0.1" # replace with the latest version on crates.io
```

In tests:

```rust
use googletest::prelude::*;
use googletest_serde_json::json;
use serde_json::json as j;
```

> The crate re-exports a `json` namespace with everything you need:  
> - `json::value(...)` – match a value inside a JSON `Value`  
> - `json::pat!{...}` – match a JSON object by fields  
> - `json::elements_are![...]` – match arrays element-by-element (ordered)  
> - `json::unordered_elements_are![...]` – match arrays element-by-element (unordered)  
> - `json::is_contained_in![...]` – assert containment of JSON elements

---

## Quick start

### Match JSON values with `json::value`

```rust
#[test]
fn values() {
    assert_that!(j!(42),        json::value(gt(40)));             // i64
    assert_that!(j!(3.14),      json::value(close_to(3.1, 0.1))); // f64
    assert_that!(j!("hello"),   json::value(starts_with("he"))); // &str
    assert_that!(j!(true),      json::value(is_true()));          // bool
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

### Match arrays with `json::elements_are!` (ordered)

```rust
#[test]
fn arrays_ordered() {
    assert_that!(
        j!(["hello", 42, true]),
        json::elements_are![eq("hello"), eq(42), eq(true)]
    );
}
```

---

### Match arrays with `json::unordered_elements_are!` (unordered)

```rust
#[test]
fn arrays_unordered() {
    assert_that!(
        j!([42, "hello", true]),
        json::unordered_elements_are![eq("hello"), eq(42), eq(true)]
    );
}
```

---

### Assert containment with `json::is_contained_in!`

```rust
#[test]
fn containment() {
    assert_that!(
        j!({"a": 1, "b": 2, "c": 3}),
        json::is_contained_in!({
            "a": eq(1),
            "c": eq(3),
        })
    );
}
```

---

## Combined example

Compose all together for complex structures. Here is a Rust struct with a JSON field, using `matches_pattern!` for the struct and nested JSON matchers for the field:

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
                    "name": json::value(starts_with("Ali")),
                    "tags": json::elements_are![eq("admin"), eq("tester")],
                    "active": json::value(is_true()),
                    ..
                })
            })
        })
    );
}
```

---