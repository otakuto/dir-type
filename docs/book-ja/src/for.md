# `for` によるコレクションの反復

「`items/` にあるサブディレクトリと同名のディレクトリが `mirror/` にも存在すること」のような、動的な対称性の検証が必要になる場面がある。そのときは `for` を使う。

正規表現の名前付きキャプチャ（`(?<name>...)`）と `id:` でエントリを収集し、`for:` ブロックで繰り返し検証を記述できる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/for --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/for/.type-dir.yaml}}
```

## 解説

### エントリの収集

`items/` 内の各サブディレクトリを `regex: '^(?<name>[a-z]+)$'` でマッチさせ、`id: item` として収集する。名前付きキャプチャ `(?<name>...)` により、ディレクトリ名が `name` というキャプチャ変数に束縛される。

### `for` によるイテレーション

`for:` ブロックは収集した `item` のリストをイテレートする。実際のYAMLでは次の形式で書く。

```yaml
- for:
    id: it
    value: ${dir.items_dir.dir.item}
  ::
    - dir: '${value.it.regex.n}'
      ::
        - file: copy.txt
```

- `id: it`: 各イテレーションでの変数名を指定する
- `value: ${dir.items_dir.dir.item}`: `id: items_dir` を持つ `items` ディレクトリの中の `item` コレクションをイテレートする
- `${value.it.regex.name}`: 現在の要素 `it` の名前付きキャプチャ `name` を参照する

### `${...}` 展開

`${value.it.regex.name}` の形式で、収集したエントリのキャプチャ変数を文字列として展開できる。`regex.name` は名前付きキャプチャ `name` でマッチした文字列を参照する。

この例では `items/` に `x/` と `y/` があるとき、`mirror/` に `x/` と `y/` の両方が存在し、それぞれに `copy.txt` を持つことを要求する。`items/` に新しいサブディレクトリを追加すると、その瞬間 `mirror/` 側にも対応するディレクトリが必須になる。対称性の崩れをその場で検出できる。
