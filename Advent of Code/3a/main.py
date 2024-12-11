import re

res = 0
while True:
    try:
        mem = input()
    except EOFError:
        break

    it = re.finditer(r"mul\((\d{1,3}),(\d{1,3})\)", mem)
    for i in it:
        a, b = i.group(1), i.group(2)
        a, b = map(int, [a, b])
        res += a * b

print(res)
