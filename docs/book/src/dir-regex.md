# `dir` + `regex` — directory type with a pattern

Just like `file:`, you can write a mapping after `dir:` and put a regular expression in the `regex:` key to accept directories whose names match a pattern. All directories matching the regular expression are accepted, and the contents declared under `::` are applied to each of them.

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/dir-regex --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/dir-regex/.dir-lint.yaml}}
```

This rule accepts any single-lowercase-letter directory such as `a/`, `b/`, and `c/`, and requires a file named `file` to exist inside each of them.

The power of `dir:` + `regex:` is that it **applies the same rule to every directory matching the pattern**. Structures where directory names are determined dynamically — for example, directories named after version numbers or module names — can be typed with a single declaration.

Combining named capture groups (`(?<name>...)`) with `id:` lets you collect all matching directories as a collection and iterate over them with a subsequent `for`. See [for — iterate over a collection](./for.md) for details.
