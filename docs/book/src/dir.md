# `dir` — require a subdirectory to exist

A `dir` entry requires that a directory with the specified name exists. By nesting `::` inside it, you can describe the rules for that subdirectory inline. If the directory does not exist, `LT002 required name not found` is reported.

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/dir --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/dir/.dir-type.yaml}}
```

This rule requires the `src/` directory to exist and `main.txt` to exist inside it. Placing `::` immediately after `dir:` lets you describe the structure of that directory inline.

`dir:` entries can be nested arbitrarily. Deep hierarchies can be expressed simply by increasing the indentation level. Any entry found inside a directory that is not listed under `::` is reported as `LT001 undeclared path`.
