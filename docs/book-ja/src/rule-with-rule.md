# `use.` ホップによるルール渡しとレコードのパラメータ化

`use.<id>` をホップとして使うと、`use: rule.X id: <id>` で束ねた splice 結果に参照パスの途中から降りることができる。`for` のソース式に `${dir.components_root.use.component}` を指定することで、中間の wrapper id を経由せずに収集結果を直接反復できる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/rule-with-rule --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/rule-with-rule/.dir-type.yaml}}
```

## 解説

### component の捕捉

`components/` ディレクトリには `id: components_root` を付与して、ディレクトリ全体を1つのオブジェクトとして参照できるようにする。その配下で `use: rule.component` に `id: component` を付けて呼び出すことで、`component` ルールの収集結果を `component` という名前の splice 結果として束ねる。

### use. ホップによる参照

`for` のソース式 `${dir.components_root.use.component}` は、`dir.components_root` で `components_root` ディレクトリに降り、`.use.component` でその配下の splice 結果 `component` に降りることを表す。これにより、`component` ルールが収集したレコードのリストを直接反復できる。

### 各 component を docs ルールへ渡す

`for` の各反復では、`component_entry` に1つの `rule.component` レコードが束縛される。`use: rule.docs` の `with:` でこのレコードを渡す。`docs` ルール側では `type: rule.component` の単体レコードとして受け取り、`${with.component_entry.regex.name}.md` というファイルを要求する。

たとえば `components/` に `auth/` と `billing/` があれば、`docs/auth.md` と `docs/billing.md` の両方が必須になる。

このパターンにより、呼び出し元の `root` ルールで反復を制御しながら、各 component レコードを `docs` ルールへ型安全に渡すことができる。
