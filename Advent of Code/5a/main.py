childs = {}
while True:
    line = input()
    if len(line) == 0:
        break

    a, b = map(int, line.split("|"))
    if a in childs:
        childs[a].add(b)
    else:
        childs[a] = {b}


total = 0
while True:
    try:
        nums = list(map(int, input().split(",")))

        res = True
        for i in range(len(nums)):
            for j in range(0, i):
                if nums[i] in childs and nums[j] in childs[nums[i]]:
                    res = False

        if res:
            total += nums[len(nums) // 2]

    except EOFError:
        break
print(total)
