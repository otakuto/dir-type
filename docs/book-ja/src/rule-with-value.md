# `with` で値を渡す — string / number / bool / list

ルールに値パラメータを渡すことができる。宣言側では `with: [{id: <名前>, type: <型>}]` のリスト形式でパラメータを宣言し、呼出側では `with: [{id: <名前>, value: <値>}]` のリスト形式で値を指定する。これにより、同じルール定義を異なる値で再利用できる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/rule-with-value --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/rule-with-value/.dir-lint.yaml}}
```

## 解説

### prim — string と number のリテラルを渡す

`prim` ルールは `type.string` の `name` と `type.number` の `version` を宣言している。呼出側では `value: api` と `value: 2` のリテラルをそれぞれ渡す。

ルール本体では `${with.name}` がディレクトリ名に、`v${with.version}` がファイル名の一部に展開される。`root` からは `api/v2.txt` というパスが要求されることになる。

### flag — bool リテラルを渡す

`flag` ルールは `type.bool` の `enabled` を宣言している。呼出側では `value: true` のboolリテラルを渡す。

`${with.enabled}` は `true` または `false` という文字列に展開される。この例では `enabled-true.txt` というファイルが要求される。

### each — 配列を渡して for で反復する

`each` ルールは `[type.string]` の `items` を宣言している。値式(value)は内部的に string(スカラー)と string の list のみをサポートするため、配列をインラインで `value: [alpha, beta]` のように呼出側の `with:` に直接書くことはできない。

代わりに、`root` ルール内でまず value バインディング `- id: names / value: [alpha, beta]` を作り、`use: rule.each` の `with:` で `value: ${value.names}` と参照渡しする。

`each` ルール本体では `for` でその配列を反復し、各要素名のディレクトリを要求する。結果として `alpha/marker.txt` と `beta/marker.txt` の両方が必須になる。

## 補足

値式がサポートするのは string スカラーと string の list のみであり、object 値を構築する手段はない。`type: {field: T}` の形式でオブジェクト型のパラメータを宣言することは構文上可能だが、その値をリテラルやバインディングで渡せない。構造化データ(レコード)をパラメータとして渡したい場合は、次ページの「with でルールを渡す」を参照すること。
