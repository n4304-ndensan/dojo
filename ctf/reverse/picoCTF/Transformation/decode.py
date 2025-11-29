c = "灩捯䍔䙻ㄶ形楴獟楮獴㌴摟潦弸彤㔲挶戹㍽"

for i in c:
    # 改行無し
    print(chr(ord(i) >> 8), end='')  #上位8bitは右シフト演算で取得
    print(chr(ord(i) & 255), end='') #下位8bitは255とのAND演算で取得
