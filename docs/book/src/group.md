# Groups — Bundling Multiple Entries as a Record

An entry with a `group:` marker key — and no `dir:`, `file:`, or `rule:` key — is called a "group." It carries `id:` and `::` as sibling keys and bundles multiple entries at the same level together as a single record.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/group --noreport -->
```

## Rule Definition

```yaml
{{#include ../../../tutorials/group/.dir-type.yaml}}
```

## Explanation

A normal `dir:` entry groups entries by inserting a subdirectory between them. A group bundles entries at the same level into one record named `unit`, without inserting any directory in between.

In this example, `name.txt` (a file) and `data/` (a directory containing `value.txt`) are bundled together as a single record under the group `id: unit`.

The primary use of a group is iteration with `for`. When a group is expanded by `for`, repeated processing can be applied to the collected results of each entry inside the group. See `016_group-clone` for an example of this pattern in practice.
