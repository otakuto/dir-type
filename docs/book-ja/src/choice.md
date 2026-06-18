# `choice` — N〜M個を要求する

`choice` は `min` / `max` で個数範囲を指定し、列挙した選択肢のうちその個数だけが存在することを要求する。`one_of`(ちょうど 1 つ)と `any_of`(1 つ以上)の一般化にあたる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/choice --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/choice/.type-dir.yaml}}
```

このルールは `docs/` ディレクトリに `guide.md` / `api.md` / `faq.md` のうち 1 個以上 2 個以下が存在することを要求する。

`min` と `max` の両方を指定することで「N 個以上 M 個以下」という制約を表現する。`min: 1` かつ `max: 1` は `one_of` と等価になる。各エントリに `id:` を付けることで、存在したエントリの種類を後続の処理で参照できる。
