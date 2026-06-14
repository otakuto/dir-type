# Summary

- [はじめに](./introduction.md)
- [インストール](./installation.md)
- [基本操作: check と設定ファイル](./getting-started.md)

# 型の構成要素

- [file — ファイルの存在を必須とする](./file.md)
- [dir — サブディレクトリを必須とする](./dir.md)
- [optional — 存在してもしなくてもよい](./optional.md)
- [file + regex — パターンを持つファイル型](./file-regex.md)
- [dir + regex — パターンを持つディレクトリ型](./dir-regex.md)
- [min / max — 個数制限](./count.md)
- [one_of — ちょうど1つを要求する](./one-of.md)
- [any_of — 1つ以上を要求する](./any-of.md)
- [choice — N〜M個を要求する](./choice.md)
- [rule / use — 型を再利用する](./rule-use.md)
- [group — レコードをまとめて扱う](./group.md)

# 動的構造とミラーリング

- [for — コレクションをイテレートする](./for.md)
- [with で値を渡す — string / number / bool / list](./rule-with-value.md)
- [with でルールを渡す — レコードをパラメータ化する](./rule-with-rule.md)
- [match — 型に基づいて分岐する](./match.md)
- [fetch — 消費せずに収集する](./fetch.md)
- [recursive — 任意の深さの構造を型付けする](./recursive.md)
- [group-clone — 収集結果を別の場所で使う](./group-clone.md)
- [recursive-clone — ツリーをミラーする](./recursive-clone.md)
- [recursive-flatten — ツリーを平坦化する](./recursive-flatten.md)
