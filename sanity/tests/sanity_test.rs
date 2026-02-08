use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn comprehensive_matchers_demo() {
    let response = json!({
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
            "is_non_empty_array": [1],
            "is_non_empty_object": { "k": "v" },
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
            "each_is_string": ["a", "b"],
            "each_is_number": [1, 2],
            "each_is_boolean": [true, false],
            "each_is_null": [null, null],
            "each_is_array": [[1], [2]],
            "each_is_object": [{"a": 1}, {"b": 2}],
        },
        "composite_patterns": {
            "simple_pat": {
                "key1": "value1",
                "key2": 10
            },
            "matches_pattern_alias": {
                "name": "Nandor",
                "age": 758
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
            },
            "implicit_pat": {
                "outer": {
                    "inner": "ok",
                    "count": 2
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
        j::pat!({
            "type_checks": j::pat!({
                "is_null": j::is_null(),
                "is_not_null": j::is_not_null(),
                "is_number": j::is_number(),
                "is_string": j::is_string(),
                "is_empty_string": j::is_empty_string(),
                "is_non_empty_string": j::is_non_empty_string(),
                "is_boolean": j::is_boolean(),
                "is_true": j::is_true(),
                "is_false": j::is_false(),
                "is_array": j::is_array(),
                "is_object": j::is_object(),
                "is_empty_array": j::is_empty_array(),
                "is_empty_object": j::is_empty_object(),
                "is_non_empty_array": j::is_non_empty_array(),
                "is_non_empty_object": j::is_non_empty_object(),
                "is_integer": j::is_integer(),
                "is_whole_number": j::is_whole_number(),
                "is_fractional_number": j::is_fractional_number(),
                "any_value": j::any_value(),
                "primitive_number": j::primitive!(gt(0)),
                "value_literal": j::value!(eq("literal")),
                "optional_present": j::optional!(j::is_string()),
                "optional_missing": j::optional!(j::is_not_null()),
            }),
            "array_matchers": j::pat!({
                "elements_are_ordered": j::elements_are![
                    "first",
                    j::is_number(),
                    starts_with("start")
                ],
                "unordered_elements": j::unordered_elements_are![
                    j::is_boolean(),
                    "middle",
                    lt(100)
                ],
                "contains_each": j::contains_each![
                    j::is_string(),
                    j::is_not_null()
                ],
                "is_contained_in": j::is_contained_in![
                    1,
                    eq(2),
                    eq(3)
                ],
                "each_all_positive": j::each!(gt(0)),
                "each_literal": j::each!("x"),
                "len_three": j::len!(ge(3)),
                "len_literal": j::len!(3),
                "each_is_string": j::each_is_string(),
                "each_is_number": j::each_is_number(),
                "each_is_boolean": j::each_is_boolean(),
                "each_is_null": j::each_is_null(),
                "each_is_array": j::each_is_array(),
                "each_is_object": j::each_is_object(),
            }),
            "composite_patterns": j::pat!({
                "simple_pat": j::pat!({
                    "key1": eq("value1"),
                    "key2": gt(5),
                }),
                "matches_pattern_alias": j::matches_pattern!({
                    "name": starts_with("Nan"),
                    "age": gt(500),
                }),
                "nested_pat": j::pat!({
                    "outer_key": j::pat!({
                        "inner_key": j::is_boolean(),
                        "inner_array": j::elements_are![
                            j::is_string(),
                            j::is_not_null(),
                            le(10)
                        ],
                    }),
                }),
                "implicit_pat": j::pat!({
                    "outer": {
                        "inner": starts_with("o"),
                        "count": ge(2),
                    },
                }),
            }),
            "mixed_types_and_nested": j::pat!({
                "string_field": eq("test_string"),
                "number_field": eq(99),
                "boolean_field": is_true(),
                "null_field": j::is_null(),
                "array_field": j::elements_are![
                    eq(1),
                    eq("two"),
                    is_false(),
                    j::is_null()
                ],
                "object_field": j::pat!({
                    "nested_number": lt(10),
                    "nested_string": starts_with("nest"),
                    "nested_null": j::is_null(),
                    "nested_array": j::elements_are![
                        is_true(),
                        is_false(),
                        is_true()
                    ],
                    "predicate_ok": j::predicate(|v| v.as_i64().is_some_and(|n| n > 5)),
                }),
            }),
            "path_matchers": j::pat!({
                "paths_allowing_extras": j::has_paths(&[
                    "user.id",
                    "user.address.city",
                    "items.0.name",
                    "items.1.name"
                ]),
                "paths_exact": j::has_only_paths(&[
                    "only",
                    "only.a",
                    "list",
                    "list.0",
                    "list.0.k"
                ]),
                "empty_object": j::is_empty_object(),
            }),
        })
    );

    assert_that!(
        response,
        j::has_path_with!("path_matchers.paths_allowing_extras.user.role", eq("admin"))
    );
}
