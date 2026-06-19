# `file` によるファイルの必須化

**`file` エントリ**は最もシンプルなエントリである。指定した名前のファイルが存在しなければ `LT002 required name not found` を報告する。

設定ファイル自体(`.dir-type.yaml`)もルールに宣言されていないパスと見なされるため、通常はルールの中に含める必要がある。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/file --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/file/.dir-type.yaml}}
```

このルールは、対象ディレクトリ直下に `.dir-type.yaml` と `required.txt` の両方が存在することを要求する。どちらかが欠けていれば `LT002 required name not found` が報告される。

`file:` に続けて名前を直接書くと、その名前に完全一致するファイルを必須とする。複数のファイルを要求したい場合は `::` の中に並べて書けばよい。
