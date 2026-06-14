# 再帰ルール — 任意の深さの構造を型付けする

`use: rule.<名前>` を使ってルールが自分自身を参照することで、任意の深さのネスト構造を表現できる。フィーチャーディレクトリが任意の深さで入れ子になるプロジェクト構造を、1つのルール定義で検証できる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/recursive --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/recursive/.dir-lint.yaml}}
```

## 解説

### 自己再帰するルール

`feature_dir` ルールは自分自身を `use: rule.feature_dir` で参照する。`dir: feature` の中に `use: rule.feature_dir` を置くことで、`feature/` が何段ネストしていても同じルールが再帰的に適用される。

```yaml
- rule: feature_dir
  ::
    - dir:
        regex: '^[a-z0-9]+$'
      optional: true
      ::
        - file: project.txt
          optional: true
        - dir: feature
          optional: true
          ::
            - use: rule.feature_dir   # 自己再帰する
```

### `optional: true` で収束を保証する

再帰ルールでは `optional: true` が重要な役割を果たす。`dir:` も `dir: feature` もどちらも `optional: true` であるため、対応するエントリが存在しない時点で再帰が自然に止まる。実際のディレクトリツリーが有限である限り、再帰は必ず収束する。

### 表現できる構造

このルールは次の構造を許容する。

- `feature/<name>/project.txt` — 1段のフィーチャーに `project.txt` を持つ
- `feature/<name>/feature/<name>/project.txt` — 2段以上のネストにも対応する
- `project.txt` の存在は任意なので、中間ノードには `project.txt` がなくてもよい

`x/`、`y/`、`z/` のような浅いノードと、`z/feature/a/feature/0/` のような深いノードを同一のルールで検証できる。
