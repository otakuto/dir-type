# Navigating splice results with `use.` — parameterizing with records

Using `use.<id>` as a hop, you can descend directly into a splice result bundled by a `use: rule.X id: <id>` entry. By specifying `${dir.components_root.use.component}` as the source expression for `for`, you can iterate over collected results without going through intermediate wrapper ids.

## Directory structure

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/rule-with-rule --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/rule-with-rule/.dir-type.yaml}}
```

## Explanation

### Capturing components

The `components/` directory is given `id: components_root` so that the entire directory can be referenced as a single object. Inside it, `use: rule.component` is called with `id: component`, which bundles the collected results of the `component` rule under the name `component` as a splice result.

### Navigating with a `use.` hop

The for-source expression `${dir.components_root.use.component}` descends into `components_root` via `dir.components_root`, then into the splice result named `component` via the `.use.component` hop. This allows direct iteration over the list of records collected by the `component` rule.

### Passing each component to the docs rule

Each `for` iteration binds one `rule.component` record to `component_entry`. This record is passed to `use: rule.docs` via `with:`. The `docs` rule receives it as a single `type: rule.component` record and requires the file `${with.component_entry.regex.name}.md`.

For example, if `components/` contains `auth/` and `billing/`, then both `docs/auth.md` and `docs/billing.md` become mandatory.

This pattern lets the `root` rule control iteration while passing each component record to the `docs` rule in a type-safe way.
