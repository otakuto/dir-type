# `file` — ファイルの存在を必須とする

一番シンプルなエントリがこれだ。指定した名前のファイルが存在しなければ `LT002 required name not found` を報告する。

設定ファイル自体(`.dir-lint.yaml`)もルールに宣言されていないパスと見なされるため、通常はルールの中に含める必要がある。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/file --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/file/.dir-lint.yaml}}
```

このルールは、対象ディレクトリ直下に `.dir-lint.yaml` と `required.txt` の両方が存在することを要求する。どちらかが欠けていれば `LT002 required name not found` が報告される。

`file:` に続けて名前を直接書くと、その名前に完全一致するファイルを必須とする。複数のファイルを要求したい場合は `::` の中に並べて書けばよい。
