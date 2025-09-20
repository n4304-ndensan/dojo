# Glitch Cat

## Description

```txt
Author: LT 'syreal' Jones

Our flag printing service has started glitching!
Additional details will be available after launching your challenge instance.
```

## 解法（改訂版）

1. ターゲットに `nc` で接続すると、以下のような文字列が返ってきます（例）:

   ```txt
   picoCTF{gl17ch_m3_n07_' + chr(0x61) + chr(0x34) + chr(0x33) + chr(0x39) + chr(0x32) + chr(0x64) + chr(0x32) + chr(0x65) + '}
   ```

2. これは Python のような文字列連結表現で、`chr(0xNN)` はコードポイント（ここでは ASCII 値）を表します。列挙されている `chr()` の値を文字に変換して連結すればフラグが得られます。

3. 手で解釈しても良いですが、確実に再現するために Python スクリプトでデコードする例を示します：

   ```python
   # decode_flag.py
   prefix = "picoCTF{gl17ch_m3_n07_"
   codes = [0x61, 0x34, 0x33, 0x39, 0x32, 0x64, 0x32, 0x65]
   suffix = "}"

   flag = prefix + ''.join(chr(c) for c in codes) + suffix
   print(flag)
   ```

   実行例:

   ```sh
   $ python3 decode_flag.py
   picoCTF{gl17ch_m3_n07_a4392d2e}
   ```

4. 以上より、フラグは：

   ```sh
   picoCTF{gl17ch_m3_n07_a4392d2e}
   ```

### 補足（技術的背景）

- `chr(0xNN)` は Unicode のコードポイントを表すため、今回のように 0x61 等が ASCII 範囲にある場合はそのまま ASCII 文字として解釈できます。
- 出力が `chr()` で表現されている場合は「そのまま評価する（または手で変換する）」のが一番確実です。評価には Python や簡単なスクリプトを使うと安全かつ再現性があります。

## 関連技術

- 文字コード（ASCII / Unicode）
- 基本的なスクリプト言語（Python）
- 入出力解析（netcat など）
