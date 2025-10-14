# picoCTF 2024 - YaraRules0x100

## Challenge Description (English)
Dear Threat Intelligence Analyst,  
We stumbled upon a shady executable file on one of our employee's Windows PCs.  
This file managed to bypass our intrusion detection systems, suggesting it might be a new threat without existing signatures.  
Your mission is to analyze the file and create a YARA rule that will successfully detect this executable.  
Once your rule is complete, submit it to the given endpoint for validation.  
If it passes all test cases (no false positives or false negatives), you will receive the flag.

---

## 解説（日本語）

### 問題概要
この問題では、不審な `.exe` ファイルを分析し、  
**YARAルール**を作成して正確に検出（誤検出なし）することが求められました。  
提出後はサーバ側でテストされ、  
**Packed版**・**Unpacked版**の両方を検出できることが条件です。

---

### 調査
`strings` コマンドでファイルを解析すると、以下の特徴的な文字列が見つかりました：

```

UPX0
UPX1
AdjustTokenPrivileges
LookupPrivilegeValueW

```

これらから次のことがわかります：
- `UPX0`, `UPX1` は **UPX圧縮** された実行ファイルの特徴。  
- `AdjustTokenPrivileges`, `LookupPrivilegeValueW` は **Windowsの権限操作API** であり、  
  不審な動作を行うマルウェアでよく使われます。

---

###  ルール設計
誤検出（False Positive）と検出漏れ（False Negative）を両方防ぐために：
- `UPX0` がある場合 → Packed版  
- `AdjustTokenPrivileges` と `LookupPrivilegeValueW` の両方がある場合 → Unpacked版  
と2パターンを1つのルールで包括的に検出するようにしました。

---

### ✅ 最終YARAルール
```yara
import "pe"

rule YaraRules0x100
{
    meta:
        author = "shogo"

    strings:
        $str1 = "UPX0" wide ascii
        $str2 = "AdjustTokenPrivileges" wide ascii
        $str3 = "LookupPrivilegeValueW" wide ascii

    condition:
        $str1 or ($str2 and $str3)
}
```

---

### 結果

このルールを提出すると、
全テストケース（64件）を通過し、最終的に次のフラグを獲得しました：

```
picoCTF{yara_rul35_r0ckzzz_050b555a}
```

---

### 学び

* YARAでは1つのルールに複数の検出条件を組み合わせることで、
  **亜種・圧縮差異にも対応可能**。
* False Positive／False Negativeを潰すには、
  **明確な特徴文字列の組み合わせ**が重要。
* 実際のマルウェア分析でも、
  API名やセクション名から機能を推測できる点が有用。

---
