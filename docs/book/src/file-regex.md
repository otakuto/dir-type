# `file` + `regex` — file type with a pattern

When file names are dynamic, you can accept them with a regular expression. Instead of writing a scalar after `file:`, write a mapping and put the regular expression in the `regex:` key.

This accepts zero or more files matching the regular expression. It is useful as a pattern to prevent all existing files from being flagged as `LT001 undeclared path`.

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/file-regex --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/file-regex/.dir-type.yaml}}
```

This rule accepts any file matching the `<digit>.txt` pattern, such as `0.txt`, `1.txt`, and `2.txt`. The regular expression follows the syntax of Rust's `regex` crate.

Named capture groups (`(?<name>...)`) let you reference the matched substring in a subsequent `for` or `match`. For example, writing `'^(?<stem>[a-z]+)\.txt$'` lets you extract the base name of the file under the capture name `stem`.
