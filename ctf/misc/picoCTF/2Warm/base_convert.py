def base_convert(num_str: str, from_base: int, to_base: int) -> str:
    """
    任意の基数から任意の基数へ変換する関数。
    
    Args:
        num_str: 変換したい数値（文字列）
        from_base: 入力の基数（2〜36）
        to_base: 出力の基数（2〜36）

    Returns:
        変換後の数値（文字列）
    """
    # 1. 入力文字列を10進数に変換
    num = int(num_str, from_base)

    # 2. 10進数から目的の基数に変換
    digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    if num == 0:
        return "0"

    result = ""
    while num > 0:
        result = digits[num % to_base] + result
        num //= to_base

    return result
