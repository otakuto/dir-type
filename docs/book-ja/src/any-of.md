# `any_of` — 1つ以上を要求する

`any_of` の配下に列挙したエントリのうち、少なくとも 1 つが存在していなければならない。どれが存在してもよいが、1 つも存在しない場合はエラーになる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/any_of --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/any_of/.type-dir.yaml}}
```

このルールは `assets/` ディレクトリに `thumbnail_32x32.txt` / `thumbnail_64x64.txt` / `thumbnail_128x128.txt` のうち 1 個以上が存在することを要求する。

各エントリに `id:` を付けると、どのエントリが存在したかを後続の処理で参照できる。`any_of:` 自体にも `id:` を付けて収集したエントリ群を `for` でイテレートする使い方が、より高度なルールで登場する。
