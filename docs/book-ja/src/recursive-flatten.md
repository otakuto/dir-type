# `recursive-flatten` — 再帰ツリーを平坦化する

`rec/` 配下の入れ子になったツリーを、`flat/` 側でフラットに並べ替える例である。各ノードの累積パスを名前に連結し（`z-a-0` のように）、深い階層を1段の階層へと平坦化する。ネストした構造を、フラットな一覧として別の場所に再現したいときに使う。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/recursive-flatten --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/recursive-flatten/.dir-lint.yaml}}
```

## 解説

### `rec` ルールで再帰収集する

`rec` ルールは現在位置の任意名ディレクトリ群を `id: node` として収集する。各 `node` は `project.txt`（`id: project`）と子の `feature/`（`id: sub`）を任意に持てる。`feature/` 配下に `use: rule.rec` を splice することで任意の深さに対応する。

### `flatten` ルールで平坦化する

`flatten` は `with:` で `nodes`（`rec` が収集した `node` リスト）と `prefix`（累積パスのプレフィックス）を受け取る。`for` の反復変数は `id: node`（各ノード）、`id: project_entry`（`project.txt` の存在確認用）、`id: sub_entry`（子 `feature/` の存在確認用）と命名している。

```yaml
- rule: flatten
  with:
    - id: nodes
      type: rule.rec
    - id: prefix
      type: type.string
  ::
    - for:
        id: node
        value: ${with.nodes}
      ::
        - id: acc
          value: '${with.prefix}${value.node.regex.stem}'
        - for:
            id: project_entry
            value: ${value.node.file.project}
          ::
            - dir: '${value.acc}'
              ::
                - file: project.txt
        - for:
            id: sub_entry
            value: ${value.node.dir.sub}
          ::
            - use: rule.flatten
              with:
                - id: nodes
                  value: ${value.sub_entry.dir.node}
                - id: prefix
                  value: '${value.acc}-'
```

処理の流れは次のとおりである。

1. `id: acc` の `value:` で累積名を組み立てる。最初の呼び出しでは `prefix` が空なので `acc = stem`（例: `z`）になる
2. `project.txt` が存在するノード（`id: project_entry` の `for` が展開される）に対して `flat/<acc>/project.txt` を要求する
3. 子の `feature/` が存在するノード（`id: sub_entry` の `for` が展開される）に対して `prefix: '<acc>-'` で `flatten` を再帰する。再帰先では `acc = z-a`、さらにその先では `acc = z-a-0` のように累積される

### 平坦化の結果

`rec/` の階層構造と `flat/` の対応を示す。

| `rec/` のパス | `flat/` のディレクトリ |
|---|---|
| `x/project.txt` | `flat/x/project.txt` |
| `y/feature/a/project.txt` | `flat/y-a/project.txt` |
| `y/feature/b/project.txt` | `flat/y-b/project.txt` |
| `z/feature/a/feature/0/project.txt` | `flat/z-a-0/project.txt` |
| `z/feature/a/feature/1/project.txt` | `flat/z-a-1/project.txt` |
| `z/feature/b/feature/0/project.txt` | `flat/z-b-0/project.txt` |
| `z/feature/b/feature/1/project.txt` | `flat/z-b-1/project.txt` |

再帰 splice は `flat/` 直下位置で展開されるため、全ディレクトリが同じ階層に並ぶ。`for` ガード下の再帰なので、`rec` の有限ツリーに対して必ず収束する。

`recursive-clone` がツリー構造を保ったままコピーするのに対し、`recursive-flatten` はツリーをフラットな一覧へと変換する。目的に応じてどちらのパターンも選択できる。
