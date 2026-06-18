# `recursive-clone` — Mirroring a Recursive Tree to Another Location

This example takes the tree collected under `feature/` by a recursive rule and reproduces it exactly under `tests/`. Each directory name gets a `-test` suffix appended, and any node that has `project.txt` requires a corresponding `project.test.txt`. No matter how deeply the source is nested, the same hierarchical structure is enforced for the test tree. This prevents the test location from drifting away from its source structure.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/recursive-clone --noreport -->
```

## Rule Definition

```yaml
{{#include ../../../tutorials/recursive-clone/.type-dir.yaml}}
```

## Explanation

### Recursive Collection with the `feature_dir` Rule

The `feature_dir` rule uses the same self-recursive pattern as `015_recursive`, but with more detailed `id:` annotations on the collected entries.

- `id: feature_name` — collects each feature directory itself (binds the name to capture `stem`)
- `id: project` — collects `project.txt`
- `id: subfeatures` — collects child `feature/` directories

When `use: rule.feature_dir` is applied and the result is bundled as `id: feature`, you can reference the list of collected feature directories via `${use.feature.dir.feature_name}`.

### Building the Mirror with the `test_feature` Rule

`test_feature` receives a `feature` parameter via `with:`. The `with:` mechanism passes arguments to a rule, allowing the caller to supply collected entries.

```yaml
- rule: test_feature
  with:
    - id: feature
      type: rule.feature_dir
  ::
    - dir: '${with.feature.regex.stem}-test'
      ::
        - for:
            id: project
            value: ${with.feature.file.project}
          ::
            - file: project.test.txt
        - for:
            id: sub_feature_dir
            value: ${with.feature.dir.subfeatures}
          ::
            - dir: feature
              ::
                - for:
                    id: nested_feature
                    value: ${value.sub_feature_dir.dir.feature_name}
                  ::
                    - use: rule.test_feature
                      with:
                        - id: feature
                          value: ${value.nested_feature}
```

- `${with.feature.regex.stem}-test` — requires a directory name formed by appending `-test` to the stem of the passed feature
- `${with.feature.file.project}` — when `project.txt` exists, `for` iterates and requires `project.test.txt`
- `${with.feature.dir.subfeatures}` — when child `feature/` directories exist, `test_feature` is applied recursively

### Convergence of the Recursion

Because `feature_dir` uses `optional: true`, the recursion always converges as long as the actual directory tree is finite. For nodes without `project.txt`, `for: project` produces an empty iteration; when there are no child `feature/` directories, `for: sub_feature_dir` is also empty.

With this design, no matter how deeply `feature/` is nested, the corresponding structure is reproduced faithfully under `tests/` and the correspondence of test files is guaranteed.
