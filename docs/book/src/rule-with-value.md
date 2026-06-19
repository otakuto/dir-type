# Passing values with `with` — string / number / bool / list

Rules can accept value parameters. On the declaration side, parameters are declared as a list in the form `with: [{id: <name>, type: <type>}]`; on the call side, values are supplied as a list in the form `with: [{id: <name>, value: <value>}]`. This allows the same rule definition to be reused with different values.

## Directory structure

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/rule-with-value --noreport -->
```

## Rule definition

```yaml
{{#include ../../../tutorials/rule-with-value/.dir-type.yaml}}
```

## Explanation

### prim — passing string and number literals

The `prim` rule declares a `name` of `type.string` and a `version` of `type.number`. The call site passes the literals `value: api` and `value: 2` respectively.

In the rule body, `${with.name}` expands to the directory name and `v${with.version}` expands as part of the file name. From `root`, the path `api/v2.txt` will be required.

### flag — passing a bool literal

The `flag` rule declares an `enabled` parameter of `type.bool`. The call site passes the bool literal `value: true`.

`${with.enabled}` expands to the string `true` or `false`. In this example, the file `enabled-true.txt` will be required.

### each — passing an array and iterating with for

The `each` rule declares an `items` parameter of `[type.string]`. Because value expressions internally support only string scalars and lists of strings, you cannot write an array inline as `value: [alpha, beta]` directly in the call site's `with:`.

Instead, first create a value binding `- id: names / value: [alpha, beta]` inside the `root` rule, then pass it by reference as `value: ${value.names}` in the `with:` of `use: rule.each`.

In the `each` rule body, `for` iterates over the array and requires a directory for each element name. As a result, both `alpha/marker.txt` and `beta/marker.txt` become mandatory.

## Notes

Value expressions support only string scalars and lists of strings; there is no way to construct an object value. Declaring an object-typed parameter with `type: {field: T}` is syntactically valid, but its value cannot be supplied via a literal or a binding. If you need to pass structured data (a record) as a parameter, refer to the next page, "Passing rules with `with`".
