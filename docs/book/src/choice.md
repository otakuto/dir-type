# `choice` — require N to M

`choice` uses `min` / `max` to specify a count range and requires exactly that many of the listed options to be present. It is the generalization of `one_of` (exactly 1) and `any_of` (1 or more).

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/choice --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/choice/.dir-type.yaml}}
```

This rule requires between 1 and 2 of `guide.md`, `api.md`, and `faq.md` to exist in the `docs/` directory.

Specifying both `min` and `max` expresses an "at least N and at most M" constraint. `min: 1` combined with `max: 1` is equivalent to `one_of`. Adding `id:` to each entry lets you reference which entries are present in subsequent processing.
