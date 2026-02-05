use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum PathSegment {
    Field(String),
    Index(usize),
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedPath {
    pub(crate) raw: String,
    pub(crate) segments: Vec<PathSegment>,
}

pub(crate) struct ParsedPaths {
    pub(crate) parsed: Vec<ParsedPath>,
    pub(crate) errors: Vec<String>,
}

pub(crate) fn parse_expected_paths(paths: &[&str]) -> ParsedPaths {
    let mut parsed = Vec::new();
    let mut errors = Vec::new();
    for path in paths {
        match parse_path(path) {
            Ok(path) => parsed.push(path),
            Err(err) => errors.push(err),
        }
    }
    ParsedPaths { parsed, errors }
}

fn parse_path(path: &str) -> Result<ParsedPath, String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut chars = path.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                if let Some(next) = chars.next() {
                    current.push(next);
                } else {
                    return Err(format!("Invalid path {path:?}: trailing escape"));
                }
            }
            '.' => {
                push_segment(path, &mut segments, &mut current)?;
            }
            _ => current.push(ch),
        }
    }
    push_segment(path, &mut segments, &mut current)?;
    Ok(ParsedPath {
        raw: path.to_string(),
        segments,
    })
}

fn push_segment(
    path: &str,
    segments: &mut Vec<PathSegment>,
    current: &mut String,
) -> Result<(), String> {
    if current.is_empty() {
        return Err(format!("Invalid path {path:?}: empty segment"));
    }
    if let Ok(idx) = current.parse::<usize>() {
        segments.push(PathSegment::Index(idx));
    } else {
        segments.push(PathSegment::Field(current.clone()));
    }
    current.clear();
    Ok(())
}

pub(crate) fn collect_paths(value: &Value) -> BTreeSet<Vec<PathSegment>> {
    let mut paths = BTreeSet::new();
    collect_paths_inner(value, &mut Vec::new(), &mut paths);
    paths
}

fn collect_paths_inner(
    value: &Value,
    current: &mut Vec<PathSegment>,
    out: &mut BTreeSet<Vec<PathSegment>>,
) {
    if !current.is_empty() {
        out.insert(current.clone());
    }
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                current.push(PathSegment::Field(k.clone()));
                collect_paths_inner(v, current, out);
                current.pop();
            }
        }
        Value::Array(arr) => {
            for (idx, v) in arr.iter().enumerate() {
                current.push(PathSegment::Index(idx));
                collect_paths_inner(v, current, out);
                current.pop();
            }
        }
        _ => {}
    }
}

pub(crate) fn format_path(path: &[PathSegment]) -> String {
    path.iter()
        .map(|segment| match segment {
            PathSegment::Field(f) => escape_field(f),
            PathSegment::Index(i) => i.to_string(),
        })
        .collect::<Vec<_>>()
        .join(".")
}

fn escape_field(field: &str) -> String {
    let mut out = String::new();
    for ch in field.chars() {
        match ch {
            '.' => {
                out.push('\\');
                out.push('.');
            }
            '\\' => {
                out.push('\\');
                out.push('\\');
            }
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn f(name: &str) -> PathSegment {
        PathSegment::Field(name.to_string())
    }

    #[test]
    fn parse_path_accepts_nested_fields_and_indices() {
        let ParsedPaths { parsed, errors } = parse_expected_paths(&["user.id", "items.0.id"]);
        assert!(errors.is_empty());
        assert_eq!(parsed[0].segments, vec![f("user"), f("id")]);
        assert_eq!(
            parsed[1].segments,
            vec![f("items"), PathSegment::Index(0), f("id")]
        );
    }

    #[test]
    fn parse_path_supports_escaped_dot() {
        let ParsedPaths { parsed, errors } = parse_expected_paths(&[r"user\.name"]);
        assert!(errors.is_empty());
        assert_eq!(parsed[0].segments, vec![f("user.name")]);
    }

    #[test]
    fn parse_path_errors_on_empty_segment() {
        let ParsedPaths { parsed, errors } = parse_expected_paths(&["foo..bar"]);
        assert!(parsed.is_empty());
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("empty segment"));
    }

    #[test]
    fn collect_paths_traverses_objects_and_arrays() {
        let value = json!({"user": {"id": 1}, "list": [{"a": 1}]});
        let paths = collect_paths(&value);
        assert!(paths.contains(&vec![f("user")]));
        assert!(paths.contains(&vec![f("user"), f("id")]));
        assert!(paths.contains(&vec![f("list")]));
        assert!(paths.contains(&vec![f("list"), PathSegment::Index(0)]));
        assert!(paths.contains(&vec![f("list"), PathSegment::Index(0), f("a")]));
    }

    #[test]
    fn format_path_round_trips_with_escape() {
        let path = vec![f("user.name"), PathSegment::Index(0)];
        assert_eq!(format_path(&path), r"user\.name.0");
    }

    #[test]
    fn parse_path_errors_on_trailing_escape() {
        let ParsedPaths { parsed, errors } = parse_expected_paths(&["user\\"]);
        assert!(parsed.is_empty());
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("trailing escape"));
    }

    #[test]
    fn format_path_escapes_backslashes() {
        let path = vec![f("user\\name")];
        assert_eq!(format_path(&path), r"user\\name");
    }
}
