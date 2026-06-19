# `any_of` — require one or more

At least one of the entries listed under `any_of` must exist. Any combination is allowed, but having none of them present is an error.

## Directory layout

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/any_of --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/any_of/.dir-type.yaml}}
```

This rule requires at least one of `thumbnail_32x32.txt`, `thumbnail_64x64.txt`, or `thumbnail_128x128.txt` to exist in the `assets/` directory.

Adding `id:` to each entry lets you reference which entries are present in subsequent processing. A common pattern in more advanced rules is to attach `id:` to `any_of:` itself to collect the matched entries and iterate over them with `for`.
