# `with` でルールを渡す — レコードをパラメータ化する

`type: rule.<ルール名>` の形式でルール型のパラメータを宣言することができる。あるルールが収集したレコードを `value: ${...}` で別のルールに渡すことで、「ソース側で捕捉した構造を、別の場所の検証に使う」という連携が実現できる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/rule-with-rule --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/rule-with-rule/.dir-lint.yaml}}
```

## 解説

### component の捕捉

`components/` ディレクトリ内では `use: rule.component` に `id: comp` を付けて呼び出す。`component` ルールは `'^(?<name>.+)$'` の regex で各サブディレクトリを捕捉し、`id: component` でそのレコードを束ねる。`id: comp` を付けることで、`${use.comp.dir.component}` という参照パスで収集結果にアクセスできるようになる。

### doc_for へのルール渡し

`docs/` ディレクトリ内では `for` で `${use.comp.dir.component}` を反復する。各反復では捕捉された component レコード1件を `value.component_entry` として取り出し、`use: rule.doc_for` の `with:` で `value: ${value.component_entry}` と渡す。

`doc_for` ルールは `type: rule.component` の `component_entry` を宣言している。ルール本体では `${with.component_entry.regex.name}` で渡されたレコードの named capture `name` を参照し、`<name>.md` というファイルを要求する。

たとえば `components/` に `button/` と `input/` があれば、`docs/button.md` と `docs/input.md` の両方が必須になる。

### まとめ

この仕組みにより、「ソース側(components/)で捕捉した構造を、別のルール(doc_for)に渡して別の場所(docs/)の検証に使う」ことができる。ディレクトリ間の対応関係を型安全に表現したい場合に有効なパターンである。
