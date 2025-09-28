# `radare2`

## 概要

**逆アセンブル／解析／デバッグ** がこれ一つで可能。CLI 中心だが強力。

## 起動オプション（外側）

* `-A` : 起動時に自動解析（≒中で `aaa` 実行）
* `-q` : 静かに起動（バナー非表示）
* `-c "cmd;cmd"` : **起動直後にコマンド実行**
* `-d` : **デバッガ起動**（実行しながら解析）
* `-e key=value` : 設定を起動時に変更（例：`-e bin.relocs.apply=true`）

### よく使う組み合わせ

```bash
# とりあえず main を一発表示して終了
r2 -Aqc "afl; s main; pdf" ./a.out

# Relocs 警告がうるさい時は設定を付ける
r2 -q -e bin.relocs.apply=true -e bin.cache=true -c "aaa; afl; s main; pdf" ./a.out

# デバッグで main にブレークして続行
r2 -d -q -c "db main; dc" ./a.out
```

## 内部コマンド（中に入ってから）

* `aaa` : **Analyze All**（関数・参照など徹底解析）
* `afl` : 関数一覧（`main` を探す）
* `s main` : シーク（`main` へ移動）
* `pdf` : **現在関数の逆アセンブル**（print disasm function）
* `iz` / `izz` : 文字列一覧（`izz` は全体スキャンで時間かかる）
* `VV` : グラフビュー（q で戻る）
* `/ str TEXT` : 検索（現在位置から）
* `q` : 終了

### 例（中で打つ）

```text
aaa
afl
s main
pdf
iz
```

## 最小デバッグセット

```bash
r2 -d ./a.out
# 中で
db main   # ブレークポイント
dc        # 実行（continue）
ds        # 1命令ステップ
dr        # レジスタ表示
px 64 @ rsp  # スタックの中身を16進表示
```

## “迷ったらこれ” レシピ

* **骨格だけ素早く確認**：`r2 -Aqc "afl; s main; pdf" file`
* **フラグ/文字列の当たりをつける**：`r2 -Aqc "izz; iz" file` → 文字列を眺める
* **Relocs 警告が出る**：`-e bin.relocs.apply=true -e bin.cache=true` を付ける
* **動作も確認したい**：`r2 -d -qc "db main; dc" file` → 必要に応じて `ds/dr/px`

---

## 使い分けの指針

* **まず広く手早く拾う** → `strings -a -n 6 -t x`
* **仕組みを読む／裏を取る** → `radare2`（`aaa → afl → s main → pdf`）
* **実行時の挙動も見たい** → `radare2 -d`（ブレーク＆ステップ）

---
