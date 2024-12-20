patterns = input().split(", ")
input()


def rec(goal: str):
    if len(goal) == 0:
        return True

    for pattern in patterns:
        if goal.startswith(pattern):
            if rec(goal[len(pattern) :]):
                return True

    return False


total = 0
while True:
    try:
        goal = input()
        res = rec(goal)
        if res:
            total += 1
        print(patterns, goal, res)
    except EOFError:
        break

print(total)
