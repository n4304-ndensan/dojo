import hashlib

text = "babies"
# md5 hash of "clowns"
text_md5 = hashlib.md5(text.encode()).hexdigest()
print(f"MD5: {text_md5}")
