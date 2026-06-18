# `min` / `max` — count limits

You can also constrain how many entries must exist — for example "at least 2 and at most 4." Simply add `min` and/or `max` to a `file` or `dir` entry.

Specifying only `min` means "at least N," specifying only `max` means "at most N," and specifying both means "between N and M inclusive."

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/count --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/count/.type-dir.yaml}}
```

This rule requires between 2 and 4 files matching the `<digit>.txt` pattern to exist inside the `shards/` directory.

`min` / `max` are most often used together with `regex:`. Use them when you want to constrain the number of files or directories matching a specific pattern.

- If the count falls below `min`, `LT002 required name not found` is reported.
- If the count exceeds `max`, `LT001 undeclared path` is reported.
