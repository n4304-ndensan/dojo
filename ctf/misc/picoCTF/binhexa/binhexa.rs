use std::io;

/// 01文字列を u8 に変換
fn binstr_to_u8(s: &str) -> u8 {
    u8::from_str_radix(s, 2).unwrap()
}

/// u8 を 01文字列に変換（8bitゼロ埋め）
fn u8_to_binstr(x: u8) -> String {
    format!("{:08b}", x)
}

fn main() {
    let mut input = String::new();
    println!("入力例: 00001111 << 2");
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        eprintln!("形式: <8bit01> <演算子> <8bit01 or シフト量>");
        return;
    }

    let left = binstr_to_u8(parts[0]);
    let op = parts[1];
    let right_str = parts[2];

    // シフト用に right を u32 にも変換できるようにする
    let right_val: u8 = if let Ok(v) = u8::from_str_radix(right_str, 2) {
        v
    } else if let Ok(v) = right_str.parse::<u8>() {
        v
    } else {
        eprintln!("右辺は 8bit01 または整数で指定してください");
        return;
    };

    let result = match op {
        "+" => left.wrapping_add(right_val),
        "-" => left.wrapping_sub(right_val),
        "*" => left.wrapping_mul(right_val),
        "/" => {
            if right_val == 0 {
                eprintln!("0 で割ることはできません");
                return;
            }
            left / right_val
        }
        ">>" => left >> right_val,
        "<<" => left << right_val,
        "|" => left | right_val,
        "&" => left & right_val,
        _ => {
            eprintln!("未知の演算子: {}", op);
            return;
        }
    };

    println!("結果: {}", u8_to_binstr(result));
}
