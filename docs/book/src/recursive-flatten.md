# `recursive-flatten` — Flattening a Recursive Tree

This example takes a nested tree under `rec/` and rearranges it into a flat layout under `flat/`. Each node's accumulated path segments are joined into its name (e.g., `z-a-0`), collapsing deep hierarchies into a single level. Use this pattern when you want to reproduce a nested structure as a flat list in a different location.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/recursive-flatten --noreport -->
```

## Rule Definition

```yaml
{{#include ../../../tutorials/recursive-flatten/.dir-type.yaml}}
```

## Explanation

### Recursive Collection with the `rec` Rule

The `rec` rule collects arbitrarily named directories at the current position as `id: node`. Each `node` can optionally have `project.txt` (`id: project`) and a child `feature/` (`id: sub`). Splicing `use: rule.rec` under `feature/` handles arbitrary depth.

### Flattening with the `flatten` Rule

`flatten` receives `nodes` (the `node` list collected by `rec`) and `prefix` (the accumulated path prefix) via `with:`. The iteration variables for the `for` entries are named `id: node` (each node), `id: project_entry` (guards the `project.txt` existence check), and `id: sub_entry` (guards the child `feature/` existence check).

```yaml
- rule: flatten
  with:
    - id: nodes
      type: rule.rec
    - id: prefix
      type: type.string
  ::
    - for:
        id: node
        value: ${with.nodes}
      ::
        - id: acc
          value: '${with.prefix}${value.node.regex.stem}'
        - for:
            id: project_entry
            value: ${value.node.file.project}
          ::
            - dir: '${value.acc}'
              ::
                - file: project.txt
        - for:
            id: sub_entry
            value: ${value.node.dir.sub}
          ::
            - use: rule.flatten
              with:
                - id: nodes
                  value: ${value.sub_entry.dir.node}
                - id: prefix
                  value: '${value.acc}-'
```

The processing flow is as follows.

1. `id: acc` computes the accumulated name via `value:`. On the first call `prefix` is empty, so `acc = stem` (e.g., `z`).
2. For nodes that have `project.txt` (the `id: project_entry` `for` expands), `flat/<acc>/project.txt` is required.
3. For nodes that have a child `feature/` (the `id: sub_entry` `for` expands), `flatten` is called recursively with `prefix: '<acc>-'`. In the recursive call `acc = z-a`, and deeper still `acc = z-a-0`, and so on.

### Result of the Flattening

The correspondence between the `rec/` hierarchy and `flat/` is shown below.

| Path in `rec/` | Directory in `flat/` |
|---|---|
| `x/project.txt` | `flat/x/project.txt` |
| `y/feature/a/project.txt` | `flat/y-a/project.txt` |
| `y/feature/b/project.txt` | `flat/y-b/project.txt` |
| `z/feature/a/feature/0/project.txt` | `flat/z-a-0/project.txt` |
| `z/feature/a/feature/1/project.txt` | `flat/z-a-1/project.txt` |
| `z/feature/b/feature/0/project.txt` | `flat/z-b-0/project.txt` |
| `z/feature/b/feature/1/project.txt` | `flat/z-b-1/project.txt` |

Because the recursive splice is expanded at the `flat/` level, all directories appear at the same depth. The recursion is guarded by `for`, so it always converges for any finite `rec` tree.

While `recursive-clone` copies a tree while preserving its structure, `recursive-flatten` transforms the tree into a flat list. Either pattern can be chosen depending on the use case.
