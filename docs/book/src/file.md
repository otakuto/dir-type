# `file` — require a file to exist

This is the simplest entry type. If a file with the specified name does not exist, `LT002 required name not found` is reported.

The config file itself (`.dir-lint.yaml`) is treated as an undeclared path unless it is included in the rule, so it normally needs to be listed.

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/file --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/file/.dir-lint.yaml}}
```

This rule requires both `.dir-lint.yaml` and `required.txt` to exist directly under the target directory. If either is missing, `LT002 required name not found` is reported.

Writing a name directly after `file:` requires an exact match for that name. To require multiple files, list them inside `::`.
