# `fetch` — Collecting Without Consuming

Ordinary entries "consume" paths inside a directory. Once consumed, a path cannot be matched by another entry. However, there are cases where you want to validate the same set of paths from multiple perspectives. Use `fetch:` for those cases.

`fetch:` collects paths that match the entries under `::`, but does not itself consume the directory. The collected result is bound with `id:` so that subsequent rules can apply further validation to the same paths.

## Directory Layout

```text
<!-- cmdrun cd ../../../ && tree tutorials/fetch --noreport -->
```

## Rule Definition

```yaml
{{#include ../../../tutorials/fetch/.type-dir.yaml}}
```

## Explanation

This example processes paths inside `items/` in 4 stages.

### 1. Collect directory names without consuming them

```yaml
- fetch:
  id: dirs
  ::
    - dir:
        regex: '^(?<n>[a-z])$'
      ::
        - file: .keep
```

The `fetch:` block collects directories such as `a/`, `b/`, and `c/` using capture `name`, but does not consume those paths. Subsequent entries can consume the same paths again.

### 2. Classify each directory as `pair` or `single`

The collected `dirs` are iterated with `for`, and `one_of` determines whether each directory is paired with a text file of the same name (`pair`) or exists alone (`single`). The entire classification result is bundled as `id: classified`.

### 3. Validate `pair` directories

Under `pair/`, `${for.classified}` is iterated with `for` and only entries tagged `pair` are expanded via `match`. Paths classified as `pair` require both the directory and the text file; paths classified as `single` require nothing here.

### 4. Validate `single` directories

Under `single/`, `${for.classified}` is similarly iterated, and entries tagged `single` require either the directory or the text file — but not both.

Using `fetch:` enables a two-phase approach: "first learn the names of a set of directories, then apply detailed classification and validation later." The pattern is to use `fetch` upfront to identify what is there, and then use subsequent `for` blocks for the actual consumption and validation.
