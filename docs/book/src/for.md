# `for` — Iterating Over a Collection

There are situations where you need to verify dynamic symmetry, such as "every subdirectory present in `items/` must also exist in `mirror/`." Use `for` for these cases.

Named regex captures (`(?<name>...)`) together with `id:` collect entries, and a `for:` block lets you describe repeated validation.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/for --noreport -->
```

## Rule Definition

```yaml
{{#include ../../../tutorials/for/.dir-type.yaml}}
```

## Explanation

### Collecting Entries

Each subdirectory inside `items/` is matched with `regex: '^(?<name>[a-z]+)$'` and collected as `id: item`. The named capture `(?<name>...)` binds the directory name to the capture variable `name`.

### Iteration with `for`

The `for:` block iterates over the collected list of `item`. In actual YAML it is written as follows.

```yaml
- for:
    id: it
    value: ${dir.items_dir.dir.item}
  ::
    - dir: '${value.it.regex.n}'
      ::
        - file: copy.txt
```

- `id: it` — specifies the variable name for each iteration
- `value: ${dir.items_dir.dir.item}` — iterates the `item` collection inside the `items` dir that carries `id: items_dir`
- `${value.it.regex.name}` — references the named capture `name` of the current element `it`

### `${...}` Expansion

The form `${value.it.regex.name}` expands the capture variable of a collected entry as a string. `regex.name` refers to the string matched by named capture `name`.

In this example, when `items/` contains `x/` and `y/`, the rule requires that `mirror/` contains both `x/` and `y/`, each holding a `copy.txt` file. Adding a new subdirectory to `items/` immediately makes the corresponding directory in `mirror/` required. Any break in symmetry is detected on the spot.
