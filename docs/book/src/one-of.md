# `one_of` — require exactly one

Sometimes you want to declare "it must be in one of these two forms, but not both." Use `one_of` for that. Exactly one of the entries listed under it must exist.

Having both present, or neither present, is an error.

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/one_of --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/one_of/.dir-type.yaml}}
```

This rule requires each directory such as `a/` or `b/` to contain exactly one of `left.txt` or `right.txt`. Having both is an error; having neither is also an error.

Any entry type can be listed under `one_of:`. You can combine `file:`, `dir:`, and `optional:`. When you use entries with `id:` inside `one_of:`, the subsequent `match` can branch on which option was chosen.
