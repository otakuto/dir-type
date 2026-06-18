# `match` — Branching Validation by Type

Sometimes you want to require different documentation depending on whether a collected item is a service directory or a config file. Use `match` for this kind of per-type branching. Tag entries with `id:` using `any_of` / `one_of`, then branch processing with `match: ${variable}`.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/match --noreport -->
```

## Rule Definition

```yaml
{{#include ../../../tutorials/match/.type-dir.yaml}}
```

## Explanation

### Tagged Collection with `any_of`

Under `components/`, `any_of` is used to collect entries while tagging each one with `id:`.

- `id: service` — collects directories ending in `-service` and binds the stem to capture `svc`
- `id: config` — collects files ending in `.conf` and binds the stem to capture `cfg`

The entire `any_of` block is bundled as `id: items` and referenced by the subsequent `for`.

### Branching with `for` + `match`

Under `docs/`, the collected `items` are iterated with `for`, and `match` switches the required documentation based on each element's tag.

```yaml
- for:
    id: item
    value: ${dir.components.choice.items}
  ::
    - match: ${value.item}
      ::
        - group:
          id: service
          ::
            - file: '${value.item.regex.svc}.md'
        - group:
          id: config
          ::
            - file: '${value.item.regex.cfg}.md'
```

`match: ${value.item}` selects the corresponding arm based on the tag of the current element `item` (`service` or `config`). Items tagged `service` require `<svc>.md` (e.g., `auth.md`, `billing.md`); items tagged `config` require `<cfg>.md` (e.g., `db.md`).

By typing the contents of `docs/` based on the contents of the same `components/`, every new service or config added automatically makes the corresponding documentation required.
