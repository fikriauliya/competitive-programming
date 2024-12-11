import re

res = 0
do = True
while True:
    try:
        mem = input()
    except EOFError:
        break

    it = re.finditer(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))", mem)

    for i in it:
        if i.group(0) == "don't()":
            do = False
        elif i.group(0) == "do()":
            do = True
        elif do:
            a, b = map(int, i.group(2, 3))
            res += a * b

print(res)
