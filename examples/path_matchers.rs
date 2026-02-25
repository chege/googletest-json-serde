use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

fn main() {
    let value = json!({
        "user": {
            "id": 7,
            "name": "Ada",
            "address": {
                "city": "London"
            }
        },
        "items": [
            {"name": "book"},
            {"name": "pen"}
        ]
    });

    assert_that!(
        value,
        j::has_paths(&[
            "user.id",
            "user.address.city",
            "items.0.name",
            "items.1.name"
        ])
    );

    assert_that!(value, j::has_path_with!("user.name", starts_with("A")));

    assert_that!(
        value,
        j::has_only_paths(&[
            "user",
            "user.id",
            "user.name",
            "user.address",
            "user.address.city",
            "items",
            "items.0",
            "items.0.name",
            "items.1",
            "items.1.name"
        ])
    );
}
