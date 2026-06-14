use std::path::Path;

use indexmap::IndexMap;

use crate::enforce::TrialMemo;
use crate::enforce::eval::eval_node;
use crate::enforce::fixtures::empty_scope;
use crate::record_map::RecordMap;
use dir_lint_expr::{ExprEntry, ExprMatcher, ExprPattern, ExprSubtree, Quant};
use dir_lint_walk::DirTree;
use dir_lint_yaml::{EntryId, RegexPattern};

/// An id-bearing file entry with a capturing regex produces a Record in `produced["x"]`
/// whose fields contain the full match ("0") and the first capture group ("1").
#[test]
fn produced_file_id_has_capture_fields() {
    // Arrange: tree with file "foo.txt"
    let tree = DirTree {
        name: "root".to_string(),
        dirs: vec![],
        files: vec!["foo.txt".to_string()],
    };
    // file entry: regex `^(.+)\.txt$`, id "x"
    let entry = ExprEntry {
        id: Some(EntryId("x".to_string())),
        source_path: None,
        count: Quant::Default,
        matcher: ExprMatcher::File {
            pattern: ExprPattern::Regex(RegexPattern(r"^(.+)\.txt$".to_string())),
            subtree: ExprSubtree::Leaf,
        },
    };
    let entries = vec![entry];
    let scope = empty_scope();
    let rules = IndexMap::new();
    let path = Path::new("root");

    // Act
    let mut errors = Vec::new();
    let mut produced = RecordMap::new();
    eval_node(
        &tree,
        &entries,
        &scope,
        &rules,
        path,
        "test_rule",
        &mut errors,
        &mut produced,
        &mut TrialMemo::new(),
    );

    // Assert: no errors, produced["x"] has one record with fields "0"="foo.txt" and "1"="foo"
    assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    let records = produced.get("x").expect("expected produced[\"x\"]");
    assert_eq!(
        records.len(),
        1,
        "expected exactly 1 record in produced[\"x\"]"
    );
    let record = &records[0];
    assert_eq!(
        record.fields.get("0").map(|s| s.as_str()),
        Some("foo.txt"),
        "expected fields[\"0\"] = \"foo.txt\""
    );
    assert_eq!(
        record.fields.get("1").map(|s| s.as_str()),
        Some("foo"),
        "expected fields[\"1\"] = \"foo\""
    );
}
