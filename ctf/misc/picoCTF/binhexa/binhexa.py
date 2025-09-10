from pwn import *

# 接続先
HOST = "titan.picoctf.net"
PORT = 51613

# バイナリ文字列を数値に
def b2i(s): return int(s, 2)

# 数値をバイナリ文字列に（幅指定）
def i2b(n, width=8): return format(n, f"0{width}b")

# メイン処理
def main():
    io = remote(HOST, PORT)

    bin1 = None
    bin2 = None
    last_result = None

    while True:
        line = io.recvline().decode(errors="ignore").strip()
        if not line:
            continue
        print(line)

        # Binary Number の記録
        if line.startswith("Binary Number 1:"):
            bin1 = b2i(line.split(":")[1].strip())
        if line.startswith("Binary Number 2:"):
            bin2 = b2i(line.split(":")[1].strip())

        # 問題文に応じて演算
        if "Operation" in line and "'" in line:
            op = line.split("'")[1]
            result = None

            if op == "&":
                result = bin1 & bin2
            elif op == "|":
                result = bin1 | bin2
            elif op == "^":
                result = bin1 ^ bin2
            elif op == "+":
                result = bin1 + bin2
            elif op == "-":
                result = bin1 - bin2
            elif op == "*":
                result = bin1 * bin2
            elif op == ">>":
                # 「by X bits」を取得
                shift = int(line.split("by")[1].split()[0])
                result = bin2 >> shift if "Number 2" in line else bin1 >> shift
            elif op == "<<":
                shift = int(line.split("by")[1].split()[0])
                result = bin2 << shift if "Number 2" in line else bin1 << shift

            last_result = result

        # バイナリ結果を求められた場合
        if "Enter the binary result" in line:
            width = max(8, last_result.bit_length())
            io.sendline(i2b(last_result, width=width))

        # 最後の16進数
        if "Enter the results of the last operation in hexadecimal" in line:
            io.sendline(format(last_result, "X"))  # 大文字HEX

        # 終了判定
        if "picoCTF{" in line:
            break

    io.close()

if __name__ == "__main__":
    main()
