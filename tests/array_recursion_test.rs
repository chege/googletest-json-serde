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

#[test]
fn long_object_patterns_match_at_the_default_limit() -> googletest::Result<()> {
    let object = (0..127)
        .map(|n| (format!("k{n}"), json!(0)))
        .collect::<serde_json::Map<_, _>>();
    verify_that!(
        serde_json::Value::Object(object),
        j::pat!({
            "k0": 0, "k1": 0, "k2": 0, "k3": 0, "k4": 0, "k5": 0, "k6": 0, "k7": 0, "k8": 0, "k9": 0,
            "k10": 0, "k11": 0, "k12": 0, "k13": 0, "k14": 0, "k15": 0, "k16": 0, "k17": 0, "k18": 0, "k19": 0,
            "k20": 0, "k21": 0, "k22": 0, "k23": 0, "k24": 0, "k25": 0, "k26": 0, "k27": 0, "k28": 0, "k29": 0,
            "k30": 0, "k31": 0, "k32": 0, "k33": 0, "k34": 0, "k35": 0, "k36": 0, "k37": 0, "k38": 0, "k39": 0,
            "k40": 0, "k41": 0, "k42": 0, "k43": 0, "k44": 0, "k45": 0, "k46": 0, "k47": 0, "k48": 0, "k49": 0,
            "k50": 0, "k51": 0, "k52": 0, "k53": 0, "k54": 0, "k55": 0, "k56": 0, "k57": 0, "k58": 0, "k59": 0,
            "k60": 0, "k61": 0, "k62": 0, "k63": 0, "k64": 0, "k65": 0, "k66": 0, "k67": 0, "k68": 0, "k69": 0,
            "k70": 0, "k71": 0, "k72": 0, "k73": 0, "k74": 0, "k75": 0, "k76": 0, "k77": 0, "k78": 0, "k79": 0,
            "k80": 0, "k81": 0, "k82": 0, "k83": 0, "k84": 0, "k85": 0, "k86": 0, "k87": 0, "k88": 0, "k89": 0,
            "k90": 0, "k91": 0, "k92": 0, "k93": 0, "k94": 0, "k95": 0, "k96": 0, "k97": 0, "k98": 0, "k99": 0,
            "k100": 0, "k101": 0, "k102": 0, "k103": 0, "k104": 0, "k105": 0, "k106": 0, "k107": 0, "k108": 0, "k109": 0,
            "k110": 0, "k111": 0, "k112": 0, "k113": 0, "k114": 0, "k115": 0, "k116": 0, "k117": 0, "k118": 0, "k119": 0,
            "k120": 0, "k121": 0, "k122": 0, "k123": 0, "k124": 0, "k125": 0, "k126": 0,
        })
    )?;
    verify_that!(
        json!({"a": 1, "b": [null, {"key": true}], "c": {"nested": 2}, "extra": 0}),
        j::pat!({"a": 1, "b": [null, {"key": true}], "c": {"nested": 2}, ..})
    )
}

#[test]
fn long_complex_array_expressions_work_in_all_entry_points() -> googletest::Result<()> {
    macro_rules! check {
        ($($n:literal),*) => {{
            let expected = json!((0..120).collect::<Vec<_>>());
            verify_that!(json!([expected]), j::each!([$(std::convert::identity($n)),*]))?;
            verify_that!(json!({"values": expected}), j::pat!({"values": [$(std::convert::identity($n)),*]}))?;
            verify_that!(json!([expected]), j::elements_are![[$(std::convert::identity($n)),*]])?;
            verify_that!(json!([expected]), j::unordered_elements_are![[$(std::convert::identity($n)),*]])?;
            verify_that!(json!([expected]), j::contains_each![[$(std::convert::identity($n)),*]])?;
            verify_that!(json!([expected]), j::is_contained_in![[$(std::convert::identity($n)),*]])?;
            verify_that!(json!([expected]), j::subset_of![[$(std::convert::identity($n)),*]])?;
            verify_that!(json!([expected]), j::superset_of![[$(std::convert::identity($n)),*]])?;
        }};
    }
    check!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119
    );
    Ok(())
}

#[test]
fn long_complex_argument_lists_work_in_all_array_macros() -> googletest::Result<()> {
    macro_rules! check {
        ($($n:literal),*) => {{
            let expected = json!((0..260).collect::<Vec<_>>());
            verify_that!(expected, j::elements_are![$(std::convert::identity($n)),*])?;
            verify_that!(expected, j::unordered_elements_are![$(std::convert::identity($n)),*])?;
            verify_that!(expected, j::contains_each![$(std::convert::identity($n)),*])?;
            verify_that!(expected, j::is_contained_in![$(std::convert::identity($n)),*])?;
            verify_that!(expected, j::subset_of![$(std::convert::identity($n)),*])?;
            verify_that!(expected, j::superset_of![$(std::convert::identity($n)),*])?;
        }};
    }
    check!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
        131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148,
        149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
        167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
        185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
        203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
        221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238,
        239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256,
        257, 258, 259
    );
    Ok(())
}

#[test]
fn complex_batches_preserve_json_groups_and_evaluation_order() -> googletest::Result<()> {
    let mut next = 0;
    let mut take = || {
        next += 1;
        next
    };
    let matcher = j::elements_are![[
        take(), take(), {"key": true}, take(), take(), take(), [null, {"x": 1}],
        take(), take(), take(), take(), [2; 2]
    ]];
    verify_that!(next, eq(9))?;
    verify_that!(
        json!([[1, 2, {"key": true}, 3, 4, 5, [null, {"x": 1}], 6, 7, 8, 9, [2, 2]]]),
        matcher
    )?;
    verify_that!(
        json!([1, 2, [{"key": true}, 2], 3, 4, [{"key": true}, 2]]),
        j::elements_are![std::convert::identity(1), std::convert::identity(2), [{"key": true}, 2],
            std::convert::identity(3), std::convert::identity(4), [{"key": true}, 2]]
    )?;
    verify_that!(
        json!([[1, 2, 3, {"key": true}], 1, 2, 3, [{"key": true}, 2]]),
        j::elements_are![
            [std::convert::identity(1), std::convert::identity(2), std::convert::identity(3), {"key": true}],
            std::convert::identity(1), std::convert::identity(2), std::convert::identity(3), [{"key": true}, 2]
        ]
    )
}
