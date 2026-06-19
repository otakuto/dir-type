# Groups and `choice` — Requiring Only the Pairs Present in Both Sides

Combining `fetch`-collected stems iterated with `for`, the trial realization of `choice`, and a `group` (record-intro) lets you type only the pairs whose two parts are *both* present (an intersection) at a separate location.

In this example, only the stems for which both `<stem>_left.txt` and `<stem>_right.txt` exist under `bundle/` require the file `pair/<stem>.txt`.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/group-clone --noreport -->
```

The state of each stem under `bundle/` is as follows.

- `a` — both `a_left.txt` and `a_right.txt` present → `pair/a.txt` is required
- `b` — only `b_left.txt` (orphan) → `pair/b.txt` is not required
- `c` — both `c_left.txt` and `c_right.txt` present → `pair/c.txt` is required
- `d` — only `d_right.txt` (orphan) → `pair/d.txt` is not required

The orphans `b_left.txt` and `d_right.txt` are allowed to exist under `bundle/` and do not cause errors.

## Rule Definition

```yaml
{{#include ../../../tutorials/group-clone/.dir-type.yaml}}
```

## Explanation

### Collecting Left Stems with `fetch`

The files matching `^(?<stem>[a-z]+)_left\.txt$` under `bundle/` are observed non-consumingly with `fetch`, collecting their stems as `id: lefts`. Because `fetch` does not consume children, later entries can still handle the same files.

```yaml
- fetch:
  id: lefts
  ::
    - file:
        regex: '^(?<stem>[a-z]+)_left\.txt$'
```

### Realizing Only the Stems Present on Both Sides via `choice`

For each collected left stem, `group: matched` requires both `<stem>_left.txt` and `<stem>_right.txt`. Placing this as an alternative of a `choice` (`min: 0`, `max: 1`) means only the stems that have both realize, while orphan stems realize zero times without becoming a violation (this expresses the intersection).

```yaml
- for:
    id: candidate
    value: ${fetch.lefts}
  id: pairs
  ::
    - choice:
      id: both
      min: 0
      max: 1
      ::
        - group:
          id: matched
          ::
            - file: '${value.candidate.regex.stem}_left.txt'
            - file: '${value.candidate.regex.stem}_right.txt'
```

For a stem whose `<stem>_right.txt` is missing (e.g. `b`), `group: matched` fails to realize with MissingRequired. The `min: 0` on the `choice` permits this, so that stem produces zero records under `both`.

### Allowing Orphan Files

Because neither `fetch` nor the `choice` trial consumes any files, the files directly under `bundle/` remain unconsumed. The `optional: true` regexes consume them, allowing all left/right files including the orphans (`b_left.txt`, `d_right.txt`).

```yaml
- file:
    regex: '^[a-z]+_left\.txt$'
  optional: true
- file:
    regex: '^[a-z]+_right\.txt$'
  optional: true
```

### Requiring Only the Matched Stems in `pair/`

Under `pair/`, the `pairs` records from `bundle` are iterated. Each candidate record carries the stem field (`regex.stem`). Iterating `${value.cand.choice.both.group.matched}` with the inner `for` loops exactly once for stems present on both sides, requiring `pair/<stem>.txt`. For an orphan stem whose `matched` is empty, the inner `for` loops zero times and emits no requirement.

```yaml
- dir: pair
  ::
    - for:
        id: cand
        value: ${dir.bundle.choice.pairs}
      ::
        - for:
            id: ok
            value: ${value.cand.choice.both.group.matched}
          ::
            - file: '${value.cand.regex.stem}.txt'
```

When the state of a pair under `bundle/` changes, the requirement under `pair/` changes accordingly. For instance, deleting `bundle/a_right.txt` makes the `a` pair incomplete, turning `pair/a.txt` into an undeclared, extra file (LT001). The intersection of pairs present on both sides is expressed by combining `fetch`, `choice`, and `group`.
