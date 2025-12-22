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
            "is_empty_string": "",
            "is_non_empty_string": "a non empty string",
            "is_boolean": true,
            "is_true": true,
            "is_false": false,
            "is_array": [1, 2, 3],
            "is_object": { "a": 1 },
            "is_empty_array": [],
            "is_empty_object": {},
            "is_integer": 7,
            "is_whole_number": 7.0,
            "is_fractional_number": 7.5,
            "any_value": "non-null",
            "primitive_number": 12,
            "value_literal": "literal",
            "optional_present": "val",
            "optional_missing": null,
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
            "each_all_positive": [1, 2, 3, 4],
            "len_three": ["a", "b", "c"],
            "each_literal": ["x", "x", "x"],
            "len_literal": [10, 20, 30],
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
                ],
                "predicate_ok": 8
            }
        },
        "path_matchers": {
            "paths_allowing_extras": {
                "user": {
                    "id": 1,
                    "address": { "city": "Oslo" },
                    "role": "admin"
                },
                "items": [
                    { "name": "item-0" },
                    { "name": "item-1" }
                ],
                "extra": true
            },
            "paths_exact": {
                "only": { "a": 1 },
                "list": [
                    { "k": "v" }
                ]
            },
            "empty_object": {}
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
                "is_empty_string": json::is_empty_string(),
                "is_non_empty_string": json::is_non_empty_string(),
                "is_boolean": json::is_boolean(),
                "is_true": json::is_true(),
                "is_false": json::is_false(),
                "is_array": json::is_array(),
                "is_object": json::is_object(),
                "is_empty_array": json::is_empty_array(),
                "is_empty_object": json::is_empty_object(),
                "is_integer": json::is_integer(),
                "is_whole_number": json::is_whole_number(),
                "is_fractional_number": json::is_fractional_number(),
                "any_value": json::any_value(),
                "primitive_number": json::primitive!(gt(0)),
                "value_literal": json::value!(eq("literal")),
                "optional_present": json::optional!(json::is_string()),
                "optional_missing": json::optional!(json::is_not_null()),
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
                "each_all_positive": json::each!(gt(0)),
                "each_literal": json::each!("x"),
                "len_three": json::len!(ge(3)),
                "len_literal": json::len!(3),
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
                    "predicate_ok": json::predicate(|v| v.as_i64().is_some_and(|n| n > 5)),
                }),
            }),
            "path_matchers": json::pat!({
                "paths_allowing_extras": json::has_paths(&[
                    "user.id",
                    "user.address.city",
                    "items.0.name",
                    "items.1.name"
                ]),
                "paths_exact": json::has_only_paths(&[
                    "only",
                    "only.a",
                    "list",
                    "list.0",
                    "list.0.k"
                ]),
                "empty_object": json::is_empty_object(),
            }),
        })
    );

    assert_that!(
        response,
        json::has_path_with!("path_matchers.paths_allowing_extras.user.role", eq("admin"))
    );
}
