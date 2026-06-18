# `optional` による省略可能なエントリの宣言

`optional: true` を付けると、そのエントリは存在しなくても許可される。存在した場合は `::` で指定した内容に従って検証される。

`optional:` を使わない場合、`file:` や `dir:` エントリはすべて必須扱いになる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/optional --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/optional/.type-dir.yaml}}
```

`extra.txt` には `optional: true` が付いているため、存在しなくてもエラーにならない。一方で `required.txt` には `optional:` がないため、存在しなければ `LT002 required name not found` が報告される。

`optional: true` を付けたエントリは「存在したらそのエントリとして受理する」という意味を持つ。宣言されていない名前のファイルがディレクトリに存在した場合は、`optional:` の有無にかかわらず `LT001 undeclared path` が報告される。「宣言されているが存在しなくてもよい」という意味であり、「宣言なしでも存在してよい」という意味ではない。
