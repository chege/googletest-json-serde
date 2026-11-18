use googletest::prelude::*;
use googletest_json_serde::json as j;
use serde_json::json;

#[test]
fn long_json_arrays_work_in_patterns_and_array_matchers() -> googletest::Result<()> {
    let expected = json!((0..100).collect::<Vec<_>>());
    verify_that!(
        json!({"values": expected}),
        j::pat!({"values": [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
            40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
            50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
            60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
            70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
            80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
        ]})
    )?;
    verify_that!(
        json!([expected]),
        j::elements_are![[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
        ]]
    )
}

#[test]
fn long_mixed_arrays_preserve_json_and_expression_semantics() -> googletest::Result<()> {
    let expected: Vec<_> = (0..25)
        .flat_map(|_| vec![json!(null), json!({"k": true}), json!([2, 2]), json!(-1)])
        .collect();
    verify_that!(
        json!([expected]),
        j::each!([
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1, null, {"k": true}, [2; 2], -1,
            null, {"k": true}, [2; 2], -1,
        ])
    )
}

#[test]
fn repeat_expressions_and_nested_matchers_still_work() -> googletest::Result<()> {
    verify_that!(
        json!([[3, 3], [1, "x"]]),
        j::elements_are![
            [std::cmp::max(1, 3) + std::cmp::min(0, 0); 2],
            j::elements_are![gt(0), "x"]
        ]
    )
}
