# endianness

## 問題情報

- **Author:** Nana Ama Atombo-Sackey
- **Description:**
  Know of little and big endian? Source(flag.c)

  Additional details will be available after launching your challenge instance.

---

## 解法

1. `nc titan.picoctf.net 50277` でアプリケーションに接続。
2. 公開されている `flag.c` を確認すると、ランダムに生成された文字列に対して

   - **リトルエンディアンのバイト列文字列**
   - **ビッグエンディアンのバイト列文字列**
     の入力を待ち受けていることが分かった。

3. 入力を与えるために、文字列をエンディアン表現に変換するツール **word_to_endian.rs** を作成。

   - Little endian: 文字列を逆順にし、各文字を ASCII コードの 16 進数に変換して連結。
   - Big endian: 文字列を順番通りに 16 進数へ変換して連結。

4. それぞれの正しいバイト列文字列を入力することで、最終的に **flag** を取得できた。

---

## 関連技術

- ビッグエンディアン / リトルエンディアンの理解
- ASCII コード → 16 進数変換
- `nc` を使ったサーバとの対話

---
