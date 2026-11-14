use googletest::Result;
use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;
use std::cell::RefCell;

#[test]
fn array_matchers_accept_nested_repeat_array_expressions() -> Result<()> {
    let another = || [2, 2];
    let actual = json!([[[1, 1], [2, 2]]]);
    verify_that!(actual, j::elements_are![[[1; 2], another()]])?;
    verify_that!(actual, j::unordered_elements_are![[[1; 2], another()]])?;
    verify_that!(actual, j::contains_each![[[1; 2], another()]])?;
    verify_that!(actual, j::is_contained_in![[[1; 2], another()]])?;
    verify_that!(actual, j::subset_of![[[1; 2], another()]])?;
    verify_that!(actual, j::superset_of![[[1; 2], another()]])?;
    verify_that!(actual, j::each!([[1; 2], another()]))
}

#[test]
fn pat_matches_nested_repeat_array_expressions() -> Result<()> {
    verify_that!(
        json!({"values": [[1, 1], [2, 2]]}),
        j::pat!({"values": [[1; 2], [2; 2]]})
    )
}

#[test]
fn array_matchers_accept_array_literals_before_complex_expressions() -> Result<()> {
    let number = || 3;
    let actual = json!([[1, "x"], [true, null], 3, 1]);
    verify_that!(
        actual,
        j::elements_are![[1, "x"], [true, null], number(), [1, 2][0]]
    )?;
    verify_that!(
        actual,
        j::unordered_elements_are![[1, "x"], [true, null], number(), [1, 2][0]]
    )?;
    verify_that!(
        actual,
        j::contains_each![[1, "x"], [true, null], number(), [1, 2][0]]
    )?;
    verify_that!(
        actual,
        j::is_contained_in![[1, "x"], [true, null], number(), [1, 2][0]]
    )?;
    verify_that!(
        actual,
        j::subset_of![[1, "x"], [true, null], number(), [1, 2][0]]
    )?;
    verify_that!(
        actual,
        j::superset_of![[1, "x"], [true, null], number(), [1, 2][0]]
    )?;
    verify_that!(json!([1, 1]), j::each!([1, 2][0]))
}

#[test]
fn array_matchers_accept_owned_mixed_type_array_variables() -> Result<()> {
    let values = [json!(1), json!("x")];
    let actual = json!([[1, "x"]]);
    verify_that!(actual, j::elements_are![values.clone()])?;
    verify_that!(actual, j::unordered_elements_are![values.clone()])?;
    verify_that!(actual, j::contains_each![values.clone()])?;
    verify_that!(actual, j::is_contained_in![values.clone()])?;
    verify_that!(actual, j::subset_of![values.clone()])?;
    verify_that!(actual, j::superset_of![values.clone()])?;
    verify_that!(actual, j::each!(values))
}

#[test]
fn array_matchers_accept_repeat_arrays_with_constant_lengths() -> Result<()> {
    const COUNT: usize = 2;
    let actual = json!([[2, 2]]);
    verify_that!(actual, j::elements_are![[2; COUNT]])?;
    verify_that!(actual, j::unordered_elements_are![[2; COUNT]])?;
    verify_that!(actual, j::contains_each![[2; COUNT]])?;
    verify_that!(actual, j::is_contained_in![[2; COUNT]])?;
    verify_that!(actual, j::subset_of![[2; COUNT]])?;
    verify_that!(actual, j::superset_of![[2; COUNT]])?;
    verify_that!(actual, j::each!([2; COUNT]))
}

#[test]
fn array_matchers_evaluate_array_literal_values_once_in_order() -> Result<()> {
    let calls = RefCell::new(Vec::new());
    macro_rules! verify_matcher {
        ($name:ident) => {{
            calls.borrow_mut().clear();
            let matcher = j::$name![
                [
                    ({
                        calls.borrow_mut().push(1);
                        1
                    }),
                    ({
                        calls.borrow_mut().push(2);
                        "x"
                    })
                ],
                {
                    calls.borrow_mut().push(3);
                    3
                }
            ];
            verify_that!(calls.borrow().as_slice(), eq(&[1, 2, 3]))?;
            verify_that!(json!([[1, "x"], 3]), matcher)?;
        }};
    }
    verify_matcher!(elements_are);
    verify_matcher!(unordered_elements_are);
    verify_matcher!(contains_each);
    verify_matcher!(is_contained_in);
    verify_matcher!(subset_of);
    verify_matcher!(superset_of);
    calls.borrow_mut().clear();
    let matcher = j::each!([
        ({
            calls.borrow_mut().push(1);
            1
        }),
        ({
            calls.borrow_mut().push(2);
            "x"
        })
    ]);
    verify_that!(calls.borrow().as_slice(), eq(&[1, 2]))?;
    verify_that!(json!([[1, "x"]]), matcher)
}

#[test]
fn array_matchers_combine_array_literals_and_matchers() -> Result<()> {
    let actual = json!([[1, "x"], 3]);
    verify_that!(actual, j::elements_are![[1, "x"], gt(0)])?;
    verify_that!(actual, j::unordered_elements_are![[1, "x"], gt(0)])?;
    verify_that!(actual, j::contains_each![[1, "x"], gt(0)])?;
    verify_that!(actual, j::is_contained_in![[1, "x"], gt(0)])
}

#[test]
fn array_matchers_report_mismatched_object_array_values() -> Result<()> {
    macro_rules! verify_mismatch {
        ($name:ident) => {{
            let result = verify_that!(json!([[{"a": 2}]]), j::$name![[{"a": 1}]]);
            verify_that!(
                result,
                err(displays_as(contains_substring(r#"Array [Object {"a": Number(1)}]"#)))
            )?;
        }};
    }
    verify_mismatch!(elements_are);
    verify_mismatch!(unordered_elements_are);
    verify_mismatch!(contains_each);
    verify_mismatch!(is_contained_in);
    verify_mismatch!(subset_of);
    verify_mismatch!(superset_of);
    let result = verify_that!(json!([[{"a": 2}]]), j::each!([{"a": 1}]));
    verify_that!(
        result,
        err(displays_as(contains_substring(
            r#"Array [Object {"a": Number(1)}]"#
        )))
    )
}

#[test]
fn array_matchers_match_arrays_with_objects() -> Result<()> {
    let value = json!([[{"a": 1}, "x"], [[{"b": null}], 2], [3, {"c": true}]]);
    verify_that!(
        value,
        j::elements_are![[{"a": 1}, "x"], [[{"b": null}], 2], [3, {"c": true}]]
    )?;
    verify_that!(
        value,
        j::unordered_elements_are![[{"a": 1}, "x"], [[{"b": null}], 2], [3, {"c": true}]]
    )?;
    verify_that!(
        value,
        j::contains_each![[{"a": 1}, "x"], [[{"b": null}], 2], [3, {"c": true}]]
    )?;
    verify_that!(
        value,
        j::is_contained_in![[{"a": 1}, "x"], [[{"b": null}], 2], [3, {"c": true}]]
    )?;
    verify_that!(
        value,
        j::subset_of![[{"a": 1}, "x"], [[{"b": null}], 2], [3, {"c": true}]]
    )?;
    verify_that!(
        value,
        j::superset_of![[{"a": 1}, "x"], [[{"b": null}], 2], [3, {"c": true}]]
    )?;
    verify_that!(value, not(j::contains_each![[{"a": 2}, "x"]]))?;
    verify_that!(value, not(j::superset_of![[{"a": 2}, "x"]]))?;
    verify_that!(
        json!([[{"a": 1}, "x"], [{"a": 1}, "x"]]),
        j::each!([{"a": 1}, "x"])
    )
}

#[test]
fn array_matchers_accept_repeated_arrays_and_value_variables() -> Result<()> {
    let values = [json!(1), json!("x")];
    let kind = [2, 2];
    let matcher = json!([3, true]);
    let actual = json!([[1, "x"], [2, 2], [3, true]]);
    verify_that!(actual, j::elements_are![values.clone(), [2; 2], &matcher])?;
    verify_that!(
        actual,
        j::unordered_elements_are![&values, [2; 2], &matcher]
    )?;
    verify_that!(actual, j::contains_each![&values, [2; 2], &matcher])?;
    verify_that!(actual, j::is_contained_in![&values, [2; 2], &matcher])?;
    verify_that!(actual, j::subset_of![&values, [2; 2], &matcher])?;
    verify_that!(actual, j::superset_of![&values, [2; 2], &matcher])?;
    verify_that!(json!([[2, 2]]), j::each!(kind))
}

#[test]
fn array_matchers_preserve_duplicate_semantics_for_array_values() -> Result<()> {
    let duplicate = json!([[{"a": 1}], [{"a": 1}]]);
    verify_that!(duplicate, j::subset_of![[{"a": 1}]])?;
    verify_that!(duplicate, not(j::is_contained_in![[{"a": 1}]]))?;
    verify_that!(json!([[{"a": 1}]]), j::superset_of![[{"a": 1}], [{"a": 1}]])?;
    verify_that!(
        json!([[{"a": 1}]]),
        not(j::contains_each![[{"a": 1}], [{"a": 1}]])
    )
}

#[test]
fn array_matchers_explain_mismatched_array_literals() -> Result<()> {
    let result = verify_that!(json!([[{"a": 2}, "x"]]), j::elements_are![[{"a": 1}, "x"]]);
    verify_that!(
        result,
        err(displays_as(contains_substring(
            r#"Array [Object {"a": Number(1)}, String("x")]"#
        )))
    )
}

#[test]
fn array_matchers_accept_array_literal_expression_permutations() -> Result<()> {
    let text = String::from("x");
    let actual = json!([[-1, "x"], ["x", 1], [1, 2]]);
    verify_that!(
        actual,
        j::elements_are![[-1, "x"], [text.as_str(), 1], [1, 2,],]
    )?;
    verify_that!(
        actual,
        j::contains_each![[1, 2,], [text.as_str(), 1], [-1, "x"],]
    )?;
    verify_that!(
        actual,
        j::unordered_elements_are![[1, 2,], [text.as_str(), 1], [-1, "x"],]
    )?;
    verify_that!(
        actual,
        j::is_contained_in![[1, 2,], [text.as_str(), 1], [-1, "x"],]
    )?;
    verify_that!(
        actual,
        j::subset_of![[1, 2,], [text.as_str(), 1], [-1, "x"],]
    )?;
    verify_that!(
        actual,
        j::superset_of![[1, 2,], [text.as_str(), 1], [-1, "x"],]
    )?;
    verify_that!(json!([[-1, "x"]]), j::each!([-1, text.as_str(),]))
}

#[test]
fn pat_matches_mixed_type_array_values() -> Result<()> {
    verify_that!(
        json!({"values": [{"a": 1}, "x", null], "repeat": [2, 2]}),
        j::pat!({"values": [{"a": 1}, "x", null], "repeat": [2; 2]})
    )?;
    verify_that!(
        json!({"values": [{"a": 2}, "x", null]}),
        not(j::pat!({"values": [{"a": 1}, "x", null]}))
    )?;
    verify_that!(json!({"value": 1}), j::pat!({"value": [1, 2][0]}))
}

#[test]
fn array_matchers_accept_many_expressions() -> Result<()> {
    let zero = || 0;
    macro_rules! verify_matchers {
        ($($values:tt)*) => {{
            let mut actual = json!(vec![0; 130]);
            actual[128] = json!([{"x": 1}, true]);
            verify_that!(actual, j::elements_are![$($values)*])?;
            verify_that!(actual, j::unordered_elements_are![$($values)*])?;
            verify_that!(actual, j::contains_each![$($values)*])?;
            verify_that!(actual, j::is_contained_in![$($values)*])?;
            verify_that!(actual, j::subset_of![$($values)*])?;
            verify_that!(actual, j::superset_of![$($values)*])
        }};
    }
    macro_rules! repeat_values {
        ([]; $($values:tt)*) => {
            verify_matchers![$($values)* [{"x": 1}, true], zero()]
        };
        ([$step:tt $($rest:tt)*]; $($values:tt)*) => {
            repeat_values!([$($rest)*]; $($values)* $($values)*)
        };
    }

    repeat_values!([a b c d e f g]; zero(),)
}

#[test]
fn pat_with_mixed_array_literals_and_nested_matchers_matches() -> Result<()> {
    verify_that!(
        json!({"values": [[1, "x"], [true, null, 3], []]}),
        j::pat!({
            "values": j::elements_are![
                [1, "x"],
                j::elements_are![true, j::is_null(), gt(2)],
                []
            ]
        })
    )
}

#[test]
fn array_matchers_accept_borrowed_array_variables() -> Result<()> {
    let expected = [4, 5];
    let value = json!([[1, "x"], [4, 5]]);
    verify_that!(value, j::elements_are![[1, "x"], &expected])?;
    verify_that!(value, j::unordered_elements_are![&expected, [1, "x"]])?;
    verify_that!(value, j::contains_each![[1, "x"], &expected])?;
    verify_that!(value, j::is_contained_in![[1, "x"], &expected])?;
    verify_that!(value, j::subset_of![[1, "x"], &expected])?;
    verify_that!(value, j::superset_of![[1, "x"], &expected])?;
    verify_that!(json!([[4, 5]]), j::each!(&expected))
}

#[test]
fn pat_matches_array_value_from_rust_array() -> Result<()> {
    verify_that!(json!({"values": [1, 2]}), j::pat!({"values": [1, 2]}))
}

#[test]
fn composed_matchers_nested_object_array_optional_smoke_test() -> Result<()> {
    let value = json!({
        "user": {
            "name": "Alice",
            "age": 30,
            "nickname": null,
            "roles": ["admin", "staff"]
        },
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "user": j::pat!({
                "name": j::as_string(starts_with("Ali")),
                "age": j::as_i64(ge(18)),
                "nickname": j::optional!(starts_with("A")),
                "roles": j::elements_are![starts_with("ad"), starts_with("st")]
            }),
            "items": j::unordered_elements_are![
                j::pat!({"id": eq(2), "kind": eq("pen")}),
                j::pat!({"id": eq(1), "kind": eq("book")})
            ]
        })
    )
}

#[test]
fn pat_with_each_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "items": j::each!(j::pat!({
                "id": j::as_i64(gt(0)),
                "kind": j::as_string(anything())
            }))
        })
    )
}

#[test]
fn pat_with_each_embedded_pat_unmatches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": -2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        not(j::pat!({
            "items": j::each!(j::pat!({
                "id": j::as_i64(gt(0)),
                "kind": j::as_string(anything())
            }))
        }))
    )
}

#[test]
fn pat_with_contains_each_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"},
            {"id": 3, "kind": "notebook"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "items": j::contains_each![
                j::pat!({"kind": eq("book"), ..}),
                j::pat!({"kind": starts_with("note"), ..})
            ]
        })
    )
}

#[test]
fn pat_with_is_contained_in_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "items": j::is_contained_in![
                j::pat!({"kind": eq("book"), ..}),
                j::pat!({"kind": eq("pen"), ..}),
                j::pat!({"kind": eq("eraser"), ..})
            ]
        })
    )
}

#[test]
fn pat_with_elements_are_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "items": [
            {"id": 1, "kind": "book"},
            {"id": 2, "kind": "pen"}
        ]
    });

    verify_that!(
        value,
        j::pat!({
            "items": j::elements_are![
                j::pat!({"id": eq(1), "kind": eq("book")}),
                j::pat!({"id": eq(2), "kind": eq("pen")})
            ]
        })
    )
}

#[test]
fn pat_with_as_adapters_matches() -> Result<()> {
    let value = json!({
        "name": "Alice",
        "active": true,
        "user": {"id": "123"},
        "tags": ["admin", "staff"]
    });

    verify_that!(
        value,
        j::pat!({
            "name": j::as_string(starts_with("A")),
            "active": j::as_bool(eq(true)),
            "user": j::as_object(len(eq(1))),
            "tags": j::as_array(contains(j::as_string(eq("admin"))))
        })
    )
}

#[test]
fn unordered_elements_are_with_embedded_pat_matches() -> Result<()> {
    let value = json!([
        {"id": 1, "kind": "book"},
        {"id": 2, "kind": "pen"}
    ]);

    verify_that!(
        value,
        j::unordered_elements_are![
            j::pat!({"kind": eq("pen"), ..}),
            j::pat!({"kind": eq("book"), ..})
        ]
    )
}

#[test]
fn contains_each_with_embedded_pat_unmatches() -> Result<()> {
    let value = json!([
        {"id": 1, "kind": "book"},
        {"id": 2, "kind": "pen"}
    ]);

    verify_that!(
        value,
        not(j::contains_each![
            j::pat!({"kind": eq("book")}),
            j::pat!({"kind": eq("not-there")})
        ])
    )
}

#[test]
fn is_contained_in_with_embedded_pat_unmatches() -> Result<()> {
    let value = json!([
        {"id": 1, "kind": "book"},
        {"id": 2, "kind": "pen"},
        {"id": 3, "kind": "eraser"}
    ]);

    verify_that!(
        value,
        not(j::is_contained_in![
            j::pat!({"kind": eq("book")}),
            j::pat!({"kind": eq("pen")})
        ])
    )
}

#[test]
fn has_path_with_embedded_pat_matches() -> Result<()> {
    let value = json!({
        "user": {
            "profile": {
                "name": "Alice",
                "active": true
            }
        }
    });

    verify_that!(
        value,
        j::has_path_with!(
            "user.profile",
            j::pat!({
                "name": starts_with("Ali"),
                "active": eq(true)
            })
        )
    )
}
