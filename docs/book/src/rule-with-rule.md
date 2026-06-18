# Passing rules with `with` — parameterizing with records

You can declare a rule-typed parameter using the form `type: rule.<rule-name>`. By passing the records that one rule has captured to another rule via `value: ${...}`, you can achieve a pattern where "the structure captured on the source side is used by a separate rule to validate a different location."

## Directory structure

```text
<!-- cmdrun cd ../../../ && tree tutorials/rule-with-rule --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/rule-with-rule/.type-dir.yaml}}
```

## Explanation

### Capturing components

Inside the `components/` directory, `use: rule.component` is called with `id: comp` attached. The `component` rule captures each subdirectory using the regex `'^(?<name>.+)$'` and groups the records under `id: component`. Adding `id: comp` makes the collected results accessible via the reference path `${use.comp.dir.component}`.

### Passing a rule to doc_for

Inside the `docs/` directory, `for` iterates over `${use.comp.dir.component}`. In each iteration, one captured component record is extracted as `value.component_entry` and passed to `use: rule.doc_for` via `value: ${value.component_entry}` in `with:`.

The `doc_for` rule declares a parameter `component_entry` of `type: rule.component`. In the rule body, `${with.component_entry.regex.name}` references the named capture `name` from the passed record and requires the file `<name>.md`.

For example, if `components/` contains `button/` and `input/`, then both `docs/button.md` and `docs/input.md` become mandatory.

### Summary

This mechanism allows "the structure captured on the source side (components/) to be passed to another rule (doc_for) and used to validate a different location (docs/)." It is an effective pattern when you want to express cross-directory correspondence relationships in a type-safe manner.
