use googletest::prelude::*;
use googletest_json_serde::json;
use serde_json::json as j;

#[test]
fn comprehensive_matchers_demo() {
    let response = j!({
        "type_checks": {
            "is_null": null,
            "is_not_null": "not null",
            "is_number": 42,
            "is_string": "a string",
            "is_boolean": true,
            "is_array": [1, 2, 3],
            "is_object": { "a": 1 },
            "is_empty_array": [],
        },
        "array_matchers": {
            "elements_are_ordered": [
                "first",
                7,
                "start of something"
            ],
            "unordered_elements": [
                true,
                "middle",
                42
            ],
            "contains_each": [
                "foo",
                1,
                null
            ],
            "is_contained_in": [
                1,
                2,
                3
            ],
        },
        "composite_patterns": {
            "simple_pat": {
                "key1": "value1",
                "key2": 10
            },
            "nested_pat": {
                "outer_key": {
                    "inner_key": false,
                    "inner_array": [
                        "str",
                        5,
                        10
                    ]
                }
            }
        },
        "mixed_types_and_nested": {
            "string_field": "test_string",
            "number_field": 99,
            "boolean_field": true,
            "null_field": null,
            "array_field": [
                1,
                "two",
                false,
                null
            ],
            "object_field": {
                "nested_number": 5,
                "nested_string": "nested value",
                "nested_null": null,
                "nested_array": [
                    true,
                    false,
                    true
                ]
            }
        }
    });

    assert_that!(
        response,
        json::pat!({
            "type_checks": json::pat!({
                "is_null": json::is_null(),
                "is_not_null": json::is_not_null(),
                "is_number": json::is_number(),
                "is_string": json::is_string(),
                "is_boolean": json::is_boolean(),
                "is_array": json::is_array(),
                "is_object": json::is_object(),
                "is_empty_array": json::is_empty_array(),
            }),
            "array_matchers": json::pat!({
                "elements_are_ordered": json::elements_are![
                    "first",
                    json::is_number(),
                    starts_with("start")
                ],
                "unordered_elements": json::unordered_elements_are![
                    json::is_boolean(),
                    "middle",
                    lt(100)
                ],
                "contains_each": json::contains_each![
                    json::is_string(),
                    json::is_not_null()
                ],
                "is_contained_in": json::is_contained_in![
                    1,
                    eq(2),
                    eq(3)
                ],
            }),
            "composite_patterns": json::pat!({
                "simple_pat": json::pat!({
                    "key1": eq("value1"),
                    "key2": gt(5),
                }),
                "nested_pat": json::pat!({
                    "outer_key": json::pat!({
                        "inner_key": json::is_boolean(),
                        "inner_array": json::elements_are![
                            json::is_string(),
                            json::is_not_null(),
                            le(10)
                        ],
                    }),
                }),
            }),
            "mixed_types_and_nested": json::pat!({
                "string_field": eq("test_string"),
                "number_field": eq(99),
                "boolean_field": is_true(),
                "null_field": json::is_null(),
                "array_field": json::elements_are![
                    eq(1),
                    eq("two"),
                    is_false(),
                    json::is_null()
                ],
                "object_field": json::pat!({
                    "nested_number": lt(10),
                    "nested_string": starts_with("nest"),
                    "nested_null": json::is_null(),
                    "nested_array": json::elements_are![
                        is_true(),
                        is_false(),
                        is_true()
                    ],
                }),
            }),
        })
    );
}
