# グループと `choice` — 両方そろった組だけを別の場所に要求する

`fetch` で収集した stem を `for` でイテレートし、`choice` のトライアル realize と `group` (record-intro) を組み合わせると、「2つの要素が両方そろっている組 (intersection) だけ」を別の場所で型付けできる。

この例では、`bundle/` の中で `<stem>_left.txt` と `<stem>_right.txt` が**両方そろっている stem だけ** `pair/<stem>.txt` の存在を要求する。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/group-clone --noreport -->
```

`bundle/` の各 stem の状態は次のとおりである。

- `a` — `a_left.txt` と `a_right.txt` がそろう → `pair/a.txt` が必須
- `b` — `b_left.txt` のみ (片割れ) → `pair/b.txt` は不要
- `c` — `c_left.txt` と `c_right.txt` がそろう → `pair/c.txt` が必須
- `d` — `d_right.txt` のみ (片割れ) → `pair/d.txt` は不要

片割れの `b_left.txt`・`d_right.txt` は `bundle/` に存在してよく、エラーにはならない。

## ルール定義

```yaml
{{#include ../../../tutorials/group-clone/.dir-lint.yaml}}
```

## 解説

### `fetch` で left stem を収集する

`bundle/` の中で `^(?<stem>[a-z]+)_left\.txt$` にマッチするファイルを `fetch` で非消費観測し、その stem を `id: lefts` として収集する。`fetch` は子を consume しないため、後続のエントリが同じファイルを再度扱える。

```yaml
- fetch:
  id: lefts
  ::
    - file:
        regex: '^(?<stem>[a-z]+)_left\.txt$'
```

### `choice` のトライアルで両方そろった stem だけ realize する

収集した各 left stem について、`group: matched` で `<stem>_left.txt` と `<stem>_right.txt` の両方を要求する。これを `choice` (`min: 0`, `max: 1`) の alternative に置くことで、両方そろう stem だけが realize し、片割れの stem は 0 realize となって違反にならない (intersection の表現)。

```yaml
- for:
    id: candidate
    value: ${fetch.lefts}
  id: pairs
  ::
    - choice:
      id: both
      min: 0
      max: 1
      ::
        - group:
          id: matched
          ::
            - file: '${value.candidate.regex.stem}_left.txt'
            - file: '${value.candidate.regex.stem}_right.txt'
```

`group: matched` の中で `<stem>_right.txt` が欠落する stem (例: `b`) では、`matched` が MissingRequired で realize に失敗する。`choice` の `min: 0` がこれを許すため、その stem は `both` に 0 件の record しか産まない。

### 片割れファイルを許容する

`fetch` も `choice` のトライアルもファイルを consume しないため、`bundle/` 直下のファイルはまだ未 consume のまま残る。`optional: true` の regex でこれらを consume し、片割れ (`b_left.txt`・`d_right.txt`) を含む全ての left/right ファイルを許容する。

```yaml
- file:
    regex: '^[a-z]+_left\.txt$'
  optional: true
- file:
    regex: '^[a-z]+_right\.txt$'
  optional: true
```

### `pair/` で両方そろった stem だけ要求する

`pair/` では `bundle` の `pairs` レコードをイテレートする。各候補レコードは stem フィールド (`regex.stem`) を持つ。内側の `for` で `${value.cand.choice.both.group.matched}` をイテレートすると、両方そろった stem だけ 1 回転し、`pair/<stem>.txt` を要求する。matched が空の片割れ stem では内側 `for` が 0 回転となり、要求を出さない。

```yaml
- dir: pair
  ::
    - for:
        id: cand
        value: ${dir.bundle.choice.pairs}
      ::
        - for:
            id: ok
            value: ${value.cand.choice.both.group.matched}
          ::
            - file: '${value.cand.regex.stem}.txt'
```

`bundle/` の組の状態が変われば `pair/` の要求も連動する。例えば `bundle/a_right.txt` を削除すると `a` の組がそろわなくなり、`pair/a.txt` は宣言されない余分なファイル (LT001) になる。両方そろった組の intersection を `fetch` + `choice` + `group` の組み合わせで表現できる。
