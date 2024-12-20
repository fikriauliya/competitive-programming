patterns = input().split(", ")
input()

mem = {}


def rec(goal: str):
    if len(goal) == 0:
        return 1

    if goal in mem:
        return mem[goal]

    res = 0
    for pattern in patterns:
        if goal.startswith(pattern):
            res += rec(goal[len(pattern) :])

    mem[goal] = res

    return res


total = 0
while True:
    try:
        goal = input()
        res = rec(goal)
        total += res
        print(patterns, goal, res)
    except EOFError:
        break

print(total)
