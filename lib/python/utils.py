def chunk(data, size):
    return [data[i:i+size] for i in range(0, len(data), size)]

def xor_bytes(data: bytes, key: int) -> bytes:
    return bytes([b ^ key for b in data])
