# First Find - Writeup

## 問題文

> Unzip this archive and find the file named `uber-secret.txt`.

与えられた zip ファイルを展開し、中にある **`uber-secret.txt`** を探す問題です。

---

## 解法

### 1. zip を展開

```bash
unzip big-archive.zip -d files
```

`files/` ディレクトリに大量のファイルが展開されます。

---

### 2. `find` でファイル検索

```bash
find files -name "uber-secret.txt"
```

これで `files/.../uber-secret.txt` のパスがヒットします。

---

### 3. 中身を表示

```bash
find files -name "uber-secret.txt" -exec cat {} \;
```

またはシンプルに：

```bash
cat $(find files -name "uber-secret.txt")
```

---

## フラグ

```
picoCTF{XXXXXXXX}
```

---

## ポイント

- Linux の `find` コマンドを使えば階層が深くても一発で検索可能。
- `-exec cat {} \;` を組み合わせれば、そのままファイルの中身を確認できる。

---
