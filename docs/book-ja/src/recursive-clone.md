# `recursive-clone` による再帰ツリーのミラー

再帰ルールで収集した `feature/` 配下のツリーを、`tests/` 側にそっくり写し取る例である。各ディレクトリ名には `-test` を付け、`project.txt` を持つノードには対応する `project.test.txt` を要求する。ネストがどれだけ深くても、ソースと同じ階層構造のテストツリーを強制できる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/recursive-clone --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/recursive-clone/.type-dir.yaml}}
```

## 解説

### `feature_dir` ルールで再帰収集する

`feature_dir` ルールは前章と同じ自己再帰パターンである。ただし収集に使う `id:` を詳細に付けている。

- `id: feature_name`: 各フィーチャーディレクトリ自体を収集する（キャプチャ `stem` に名前を束縛）
- `id: project`: `project.txt` を収集する
- `id: subfeatures`: 子の `feature/` ディレクトリを収集する

`use: rule.feature_dir` した結果を `id: feature` で束ねると、`${use.feature.dir.feature_name}` で収集したフィーチャーディレクトリのリストを参照できる。

### `test_feature` ルールでミラーを構築する

`test_feature` は `with:` で `feature` パラメータを受け取る。`with:` はルールに引数を渡す仕組みで、呼び出し側が収集したエントリを渡せる。

```yaml
- rule: test_feature
  with:
    - id: feature
      type: rule.feature_dir
  ::
    - dir: '${with.feature.regex.stem}-test'
      ::
        - for:
            id: project
            value: ${with.feature.file.project}
          ::
            - file: project.test.txt
        - for:
            id: sub_feature_dir
            value: ${with.feature.dir.subfeatures}
          ::
            - dir: feature
              ::
                - for:
                    id: nested_feature
                    value: ${value.sub_feature_dir.dir.feature_name}
                  ::
                    - use: rule.test_feature
                      with:
                        - id: feature
                          value: ${value.nested_feature}
```

- `${with.feature.regex.stem}-test`: 渡されたフィーチャーの stem に `-test` を付けたディレクトリ名を要求する
- `${with.feature.file.project}`: `project.txt` が存在するとき `for` がイテレートし `project.test.txt` を要求する
- `${with.feature.dir.subfeatures}`: 子の `feature/` が存在するとき、再帰的に `test_feature` を適用する

### 再帰の収束

`feature_dir` 側が `optional: true` を持つため、実際のディレクトリツリーが有限であれば再帰は必ず収束する。`project.txt` のないノードでは `for: project` が空のイテレーションになり、子の `feature/` がなければ `for: sub_feature_dir` も空になる。

この設計により、`feature/` がどれだけ深くネストしていても、`tests/` 側に構造をそのまま写し取り、テストファイルの対応を保証できる。
