# `optional` — allow an entry to be absent

Adding `optional: true` to an entry allows it to be absent without error. When it does exist, it is validated against the contents declared under `::`.

Without `optional:`, every `file:` and `dir:` entry is treated as required. Use `optional:` when you want to declare an entry that may or may not be present.

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/optional --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/optional/.dir-lint.yaml}}
```

Because `extra.txt` has `optional: true`, its absence does not cause an error. On the other hand, `required.txt` has no `optional:`, so if it is missing, `LT002 required name not found` is reported.

An entry marked `optional: true` means "if this name is present, accept it as this entry." If a file or directory exists in the directory under a name that is not declared at all, it becomes `LT001 undeclared path` regardless of whether `optional:` is used. The key point is that `optional:` only means "declared but not required to exist."
