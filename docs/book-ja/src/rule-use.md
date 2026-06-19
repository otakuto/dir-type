# `rule` と `use` による型の再利用

同じファイル構成が複数の場所に出てくる場合、`rules:` に名前付きルールを定義し、`use: rule.<名前>` で参照することで型を再利用できる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree -a tutorials/rule --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/rule/.dir-type.yaml}}
```

## 解説

`use: rule.manifest` は参照先ルールの `::` 内容をその場に展開（splice）する。

この例では `manifest` というルールに `name.txt` と `version.txt` の2ファイルを必須とする定義をまとめている。`pkg/` ディレクトリ内に `use: rule.manifest` を書くと、`manifest` の `::` 内容がそこに展開され、`pkg/name.txt` と `pkg/version.txt` の両方が必須になる。

同じ `manifest` ルールを別のディレクトリでも参照すれば、定義を 1 か所に保ちながら複数箇所へ適用できる。繰り返し現れる構造パターンに名前を付けることで、設定ファイルを DRY に保てる。
