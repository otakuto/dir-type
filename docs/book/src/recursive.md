# Recursive Rules — Typing Structures of Arbitrary Depth

By using `use: rule.<name>` to make a rule reference itself, you can express arbitrarily deep nested structures. A project layout where feature directories nest to any depth can be validated with a single rule definition.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/recursive --noreport -->
```

## Rule Definition

```yaml
{{#include ../../../tutorials/recursive/.dir-type.yaml}}
```

## Explanation

### A Self-Recursive Rule

The `feature_dir` rule references itself with `use: rule.feature_dir`. Placing `use: rule.feature_dir` inside `dir: feature` means that the same rule is applied recursively no matter how many levels deep `feature/` is nested.

```yaml
- rule: feature_dir
  ::
    - dir:
        regex: '^[a-z0-9]+$'
      optional: true
      ::
        - file: project.txt
          optional: true
        - dir: feature
          optional: true
          ::
            - use: rule.feature_dir   # self-recursive
```

### Guaranteeing Convergence with `optional: true`

`optional: true` plays a critical role in recursive rules. Because both `dir:` and `dir: feature` are marked `optional: true`, the recursion terminates naturally when no matching entry exists. As long as the actual directory tree is finite, the recursion always converges.

### Expressible Structures

This rule allows the following structures.

- `feature/<name>/project.txt` — a single-level feature with `project.txt`
- `feature/<name>/feature/<name>/project.txt` — two or more levels of nesting
- The presence of `project.txt` is optional, so intermediate nodes do not need to have one

Shallow nodes like `x/`, `y/`, `z/` and deep nodes like `z/feature/a/feature/0/` can all be validated by the same rule.
