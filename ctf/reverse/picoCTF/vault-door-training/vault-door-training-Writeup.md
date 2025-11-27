#CTF #picoCTF #ReverseEngineering #vault-door-training

---

# Vault Door Training Writeup

## 1. 問題概要

本問題は Java ソースコードが与えられ、金庫扉のパスワード検証ロジックが `checkPassword` 関数内に直接記述されている。ユーザ入力は `picoCTF{...}` の形式を要求し、内部的に中身のみを取り出して照合している。

目的はソースコードからパスワード文字列を直接読み取り、フラグを構築することである。

---

## 2. ソースコード解析

### 入力処理

```java
String userInput = scanner.next();
String input = userInput.substring("picoCTF{".length(), userInput.length()-1);
```

入力形式は `picoCTF{<password>}`。
中括弧の終端 `}` を除いた中身のみが `input` として比較される。

### パスワード検証ロジック

```java
public boolean checkPassword(String password) {
    return password.equals("w4rm1ng_Up_w1tH_jAv4_0009yrGMeEp");
}
```

パスワードは平文でソース内に埋め込まれており、そのまま利用すればよい。

---

## 3. フラグ構築

`input` に対応するフラグは以下の形式で構築される。

```
picoCTF{<password>}
```

よって最終フラグは以下となる。

```
picoCTF{w4rm1ng_Up_w1tH_jAv4_0009yrGMeEp}
```

---

## 4. 解法の要点

* ソースコード内に平文パスワードが存在するため、リバースエンジニアリングというより静的解析問題である。
* `substring("picoCTF{".length(), input.length()-1)` より、入力は必ず `{}` で囲む必要がある。
* 比較は `equals` による完全一致であり、変換や複雑なロジックは存在しない。

---

## 5. まとめ表

| 項目   | 内容                                          |
| ---- | ------------------------------------------- |
| 分類   | ソースコード解析 / Reverse Engineering              |
| 主要概念 | 文字列処理、Java プログラム解析                          |
| 解法   | ソース中の平文パスワード抽出                              |
| フラグ  | `picoCTF{w4rm1ng_Up_w1tH_jAv4_0009yrGMeEp}` |
