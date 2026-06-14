use std::path::Path;

use indexmap::IndexMap;

use crate::enforce::TrialMemo;
use crate::enforce::eval::eval_node;
use crate::enforce::fixtures::empty_scope;
use crate::record_map::RecordMap;
use dir_lint_expr::{ExprEntry, ExprMatcher, ExprPattern, ExprSubtree, Quant};
use dir_lint_walk::DirTree;
use dir_lint_yaml::EntryId;

/// An id-less dir entry is opaque (encapsulation): child ids do NOT bubble up to the parent
/// produced map. Inner ids are reachable only by navigating through an id-bearing ancestor.
#[test]
fn produced_idless_dir_does_not_bubble_up() {
    // Arrange: tree root/src/main.rs
    let tree = DirTree {
        name: "root".to_string(),
        dirs: vec![DirTree {
            name: "src".to_string(),
            dirs: vec![],
            files: vec!["main.rs".to_string()],
        }],
        files: vec![],
    };

    // inner file entry with id "b"
    let file_entry = ExprEntry {
        id: Some(EntryId("b".to_string())),
        source_path: None,
        count: Quant::Default,
        matcher: ExprMatcher::File {
            pattern: ExprPattern::Exact("main.rs".to_string()),
            subtree: ExprSubtree::Leaf,
        },
    };
    // dir entry "src" with NO id — transparent
    let dir_entry = ExprEntry {
        id: None,
        source_path: None,
        count: Quant::Default,
        matcher: ExprMatcher::Dir {
            pattern: ExprPattern::Exact("src".to_string()),
            subtree: ExprSubtree::Inline(vec![file_entry]),
        },
    };
    let entries = vec![dir_entry];
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

    // Assert: encapsulation — the id-less dir exposes neither its own name nor its inner id `b`
    // at the parent level. Both "a" and "b" are absent from the root produced map.
    assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    assert!(
        !produced.contains_key("a"),
        "id-less dir should not create an \"a\" key in produced"
    );
    assert!(
        !produced.contains_key("b"),
        "id-less dir is opaque: inner id \"b\" must NOT bubble up to the parent produced map"
    );
}
