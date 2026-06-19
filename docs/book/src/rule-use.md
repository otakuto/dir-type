# `rule` and `use` — Reusing Types

When the same file layout appears in multiple places, rewriting the same entries every time is tedious. Define a named rule under `rules:` and reference it with `use: rule.<name>` to reuse the type. The idea is the same as type aliases in TypeScript or traits in Rust.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/rule --noreport -->
```

## Rule Definition

```yaml
{{#include ../../../tutorials/rule/.dir-type.yaml}}
```

## Explanation

`use: rule.manifest` splices the `::` contents of the referenced rule in place.

In this example, the `manifest` rule collects two required files, `name.txt` and `version.txt`. Writing `use: rule.manifest` inside the `pkg/` directory splices the `::` contents of `manifest` there, making both `pkg/name.txt` and `pkg/version.txt` required.

Referencing the same `manifest` rule from other directories lets you apply it in multiple places while keeping the definition in one spot. Any change to the rule only needs to be made once.

The combination of `rule` and `use` acts as a "type alias" for directory structure. Naming and managing recurring structural patterns keeps your configuration DRY.
