# writeup.md — Python Wrangling

## 問題

与えられた Python スクリプト `ende.py` を実行して、指定されたパスワードを使いファイルを復号し、フラグを得る問題。

* スクリプトは `-e`（暗号化） / `-d`（復号）モードをサポート。
* パスワードは引数か標準入力で与える。
* `cryptography.fernet.Fernet` を使っているが、キーの生成方法がやや独特。

## 解法（概要）

1. スクリプトの中で `Fernet` インスタンスは `base64.b64encode(sim_sala_bim.encode())` で生成された値をキーとして使っている。
2. `Fernet` が期待するのは「URL セーフな base64 エンコード済みの 32 バイト鍵（結果的に長さ44文字になる）」であるため、スクリプトが正しく動くためには入力するパスワード（`sim_sala_bim`）のバイト列が**ちょうど 32 バイト**になっているか、少なくとも `base64.b64encode(...)` の結果が `Fernet` に受け入れられる形式になっている必要がある。
3. 与えられたパスワード `192ee2db192ee2db192ee2db192ee2db` は長さ 32 の ASCII（16進っぽい文字列を繰り返したもの）で、`base64.b64encode` にかけると `Fernet` が受け取れる値になる。よってこのパスワードを使って復号すればファイルからフラグを取得できる。

## 実行手順

```bash
# 復号コマンド（例）
python ende.py -d flag.txt.en
# 実行時にパスワード入力を求められるので、以下を入力：
# 192ee2db192ee2db192ee2db192ee2db
```

実行例（提供された出力）：

```
Please enter the password:192ee2db192ee2db192ee2db192ee2db
flag
```

## コード解説（重要箇所）

該当スクリプトの要点を抜粋して説明します。

```python
ssb_b64 = base64.b64encode(sim_sala_bim.encode())
c = Fernet(ssb_b64)
```

* `sim_sala_bim` はユーザーが入力した「パスワード」文字列。
* `base64.b64encode(sim_sala_bim.encode())` はそのバイト列を base64 エンコードしたバイト列を返す。
* `Fernet` のコンストラクタは**URL セーフな base64 でエンコードされた 32 バイト鍵**（バイト列を base64 エンコードした結果：約44バイトの文字列）を受け取る期待がある。

`-e`（暗号化）と`-d`（復号）の処理：

```python
# 暗号化
with open(sys.argv[2], "rb") as f:
    data = f.read()
    data_c = c.encrypt(data)
    sys.stdout.write(data_c.decode())

# 復号
with open(sys.argv[2], "r") as f:
    data = f.read()
    data_c = c.decrypt(data.encode())
    sys.stdout.buffer.write(data_c)
```

* 暗号化では元データをバイナリで読み込み、`c.encrypt` で暗号化して標準出力に文字列として出力している。
* 復号では暗号テキストをテキストモードで読み込み `.encode()` でバイトに戻して `c.decrypt` している。暗号文は通常 ASCII 範囲（Base64 ベースで URL セーフな形式）に収まるためテキストモードでも問題は起きにくい。

### 注意点 / つまずきやすい所

* 普通は `Fernet.generate_key()` で生成したキー（44 文字の base64）をそのまま `Fernet()` に渡す。ここでは“任意のパスワードを受け取り `base64.b64encode` する”という単純化した方法を採っているため、**入力パスワードの長さや内容に依存**してしまう。
* パスワードが短い／不適切な場合、`Fernet` に渡すキーが無効になり `ValueError` 等が発生する。

## 結果（フラグ）

復号に成功し、ファイル中に書かれていたフラグは：

```
flag
```

## まとめ

* スクリプトは `Fernet` を用いたシンプルな暗号化/復号ツール。
* 鍵の扱いが独特（生のパスワードを base64 エンコードして `Fernet` に渡す）なので、正しい長さのパスワードを与えることで復号可能。
* 与えられたパスワード `192ee2db192ee2db192ee2db192ee2db` を使うと正しく復号でき、フラグが得られる。
