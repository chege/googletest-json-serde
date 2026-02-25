use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

fn main() {
    let actual = json!({
        "vampire": {
            "name": "Nandor the Relentless",
            "age": 758,
            "familiar": "Guillermo"
        },
        "house": {
            "city": "Staten Island",
            "roommates": ["Laszlo", "Nadja", "Colin Robinson"]
        }
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
            ..
        })
    );
}
