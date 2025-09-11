import base64
import re

def multi_base64_decode(b64_str: str, max_depth: int = 10) -> str:
    """Base64を多重デコードする（最大 max_depth 回まで）"""
    data = b64_str.strip()
    for _ in range(max_depth):
        try:
            decoded = base64.b64decode(data).decode("utf-8")
        except Exception:
            break

        if re.fullmatch(r"[A-Za-z0-9+/=\s]+", decoded):
            data = decoded.strip()
        else:
            return decoded
    return data

if __name__ == "__main__":
    with open("enc_flag", "r", encoding="utf-8") as f:
        b64_str = f.read()
    decoded_str = multi_base64_decode(b64_str)
    print("デコード結果:", decoded_str)
