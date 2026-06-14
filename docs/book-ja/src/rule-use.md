# `rule` と `use` — 型を再利用する

同じファイル構成が複数の場所に出てくる場合、毎回同じエントリを書き直すのは辛い。`rules:` に名前付きルールを定義し、`use: rule.<名前>` で参照すれば型の再利用ができる。TypeScript の型エイリアスや Rust のトレイトと同じ発想である。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/rule --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/rule/.dir-lint.yaml}}
```

## 解説

`use: rule.manifest` は参照先ルールの `::` 内容をその場に展開（splice）する。

この例では `manifest` というルールに `name.txt` と `version.txt` の2ファイルを必須とする定義をまとめている。`pkg/` ディレクトリ内に `use: rule.manifest` を書くと、`manifest` の `::` 内容がそこに展開され、`pkg/name.txt` と `pkg/version.txt` の両方が必須になる。

同じ `manifest` ルールを別のディレクトリでも参照すれば、定義を1か所に保ちながら複数箇所へ適用できる。ルールの変更も1か所で済む。

`rule` と `use` の組み合わせは、ディレクトリ構造の「型エイリアス」として機能する。繰り返し現れる構造パターンに名前を付けて管理することで、設定ファイルを DRY に保てる。
