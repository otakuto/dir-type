# `min` / `max` — 個数制限

「最低 2 個、最大 4 個」のような個数の制約もかけられる。`file` や `dir` エントリに `min` / `max` を付けるだけだ。

`min` のみ指定すると「少なくとも N 個」、`max` のみだと「最大 N 個」という制約になる。両方指定すると「N 個以上 M 個以下」を要求する。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/count --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/count/.type-dir.yaml}}
```

このルールは `shards/` ディレクトリの中に `<数字>.txt` 形式のファイルが 2 個以上 4 個以下存在することを要求する。

`min` / `max` は `regex:` と組み合わせて使うことが多い。特定パターンにマッチするファイルやディレクトリの個数を制約したいときに使う。

- 個数が `min` を下回ると `LT002 required name not found` が報告される
- 個数が `max` を上回ると `LT001 undeclared path` が報告される
