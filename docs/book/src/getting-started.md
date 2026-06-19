# Basics: check and the config file

## The `check` subcommand

```console
$ dir-type check [OPTIONS]
```

| Option | Short | Default | Description |
|---|---|---|---|
| `--config <PATH>` | `-c` | `.dir-type.yaml` | Path to the config file |
| `--format <FORMAT>` | — | `human` | Output format: `human` or `json` |

When the config file is `.dir-type.yaml` in the current directory, you can run without arguments.

```console
$ dir-type check
```

If validation passes, nothing is printed and the process exits with code 0. If there are violations, it exits with code 1. This makes it straightforward to use in CI `&&` chains or GitHub Actions exit-code checks.

## Config file skeleton

The config file has 3 elements.

```yaml
version: 0       # schema version (currently always 0)

entry: root      # name of the rule that serves as the validation entry point

rules:
  - rule: root   # define a rule with rule: <name>
    ::            # under ::, list the entries that may or must exist in this directory
      - file: README.md
```

`::` represents "the rules for the contents of this directory." If a path exists inside the directory that is not listed here, a `LT001 undeclared path` error is reported. In type-system terms this is equivalent to strict mode — no extra fields allowed.

As a YAML curiosity, `::` is simply a mapping entry whose key is `:`, with a value following. In JSON terms it would be `{ ":": ... }`. The `:` (key name) followed by `:` (YAML key-value separator) just happen to look like `::`. It is a YAML technique for concisely expressing a nested block.

## Integrating with CI via JSON output

With `--format json`, diagnostic information and the traversal log of the validated directory are written to stdout as JSON. Use this when you need machine-readable output in a CI pipeline.

```console
$ dir-type check --format json
```
