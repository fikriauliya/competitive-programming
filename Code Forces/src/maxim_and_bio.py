import sys

input()
s = input()
g = "ACTG"

min_c = sys.maxsize
for i in range(len(s) - len(g) + 1):
    c = 0
    for j in range(len(g)):
        # print(s[i+j], g[j], ord(s[i+j]), ord(g[j]))
        a = min([ord(s[i + j]), ord(g[j])])
        b = max([ord(s[i + j]), ord(g[j])])
        c += min([(a + 26 - b), (b - a)])
    # print(c)
    min_c = min([c, min_c])
print(min_c)
