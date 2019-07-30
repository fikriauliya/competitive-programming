input()
a = input()
fs = input().split(' ')

b = ''
start_mapping = False
end_mapping = False
for c in a:
    mapped = fs[ord(c) - ord('1')]
    if mapped > c and not end_mapping:
        b += mapped
        start_mapping = True
    elif mapped == c:
        b += c
    else:
        b += c
        if start_mapping:
            end_mapping = True
print(b)
