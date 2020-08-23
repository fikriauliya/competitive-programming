input()
s = input()

ctr  = 0
res = ""
for c in range(0, len(s), 2):
    if s[c] == s[c+1]:
        res += "ab"
        ctr += 1
    else:
        res += s[c:c+2]
print(ctr)
print(res)
