use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

fn main() {
    let names = json!(["Laszlo", "Nadja", "Colin Robinson"]);

    assert_that!(
        names,
        j::unordered_elements_are![eq("Colin Robinson"), eq("Laszlo"), eq("Nadja")]
    );

    let mixed = json!(["familiar", 1, null, true]);
    assert_that!(
        mixed,
        j::contains_each![j::is_string(), j::is_not_null(), eq(true)]
    );

    let scores = json!([7, 9, 10]);
    assert_that!(scores, j::each!(ge(7)));
    assert_that!(scores, j::len!(3));
}
