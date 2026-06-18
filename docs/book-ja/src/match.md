# `match` — 型に基づいて検証を分岐する

収集したアイテムがサービスディレクトリかコンフィグファイルかによって、要求するドキュメントを切り替えたい。そのような型ごとの分岐には `match` を使う。`any_of` / `one_of` で付与した `id:` タグを使い、`match: ${変数}` で処理を分岐できる。

## ディレクトリ構成

```text
<!-- cmdrun cd ../../../ && tree tutorials/match --noreport -->
```

## ルール定義

```yaml
{{#include ../../../tutorials/match/.type-dir.yaml}}
```

## 解説

### `any_of` でのタグ付け収集

`components/` では `any_of` を使い、各エントリに `id:` でタグを付けながら収集する。

- `id: service` — `-service` で終わるディレクトリを収集し、キャプチャ `svc` に stem を束縛する
- `id: config` — `.conf` で終わるファイルを収集し、キャプチャ `cfg` に stem を束縛する

この `any_of` ブロック全体を `id: items` として束ね、後続の `for` で参照する。

### `for` + `match` による分岐

`docs/` では収集した `items` を `for` でイテレートし、各要素のタグに応じて `match` で要求するドキュメントを切り替える。

```yaml
- for:
    id: item
    value: ${dir.components.choice.items}
  ::
    - match: ${value.item}
      ::
        - group:
          id: service
          ::
            - file: '${value.item.regex.svc}.md'
        - group:
          id: config
          ::
            - file: '${value.item.regex.cfg}.md'
```

`match: ${value.item}` は現在の要素 `item` のタグ（`service` または `config`）に応じて対応するアームを選択する。`service` タグのアイテムなら `<svc>.md`（例: `auth.md`、`billing.md`）、`config` タグのアイテムなら `<cfg>.md`（例: `db.md`）を要求する。

同じ `components/` の内容に基づいて `docs/` の内容を型付けすることで、新しいサービスやコンフィグを追加するたびに対応するドキュメントが自動的に必須になる。
