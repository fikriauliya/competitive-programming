from functools import reduce
import re
from time import sleep

# WIDTH = 11  # 101
# HEIGHT = 7  # 103
WIDTH = 101
HEIGHT = 103

ps = []
vs = []

while True:
    try:
        line = input()
        nums = re.findall(r"=((-?\d+),(-?\d+))", line)

        p = (int(nums[0][1]), int(nums[0][2]))
        v = (int(nums[1][1]), int(nums[1][2]))

        ps.append(p)
        vs.append(v)
    except EOFError:
        break


def draw(ps):
    m = [[0 for i in range(WIDTH)] for j in range(HEIGHT)]
    for p in ps:
        m[p[1]][p[0]] += 1

    for i in range(HEIGHT):
        for j in range(WIDTH):
            print(" " if m[i][j] == 0 else "1", end="")
        print()

    sums = [0, 0, 0, 0]
    for i in range(HEIGHT):
        y_q = 0
        if i > (HEIGHT // 2):
            y_q = 1
        elif i < (HEIGHT // 2):
            y_q = 1
        else:
            continue
        for j in range(WIDTH):
            x_q = 0
            if j > (WIDTH // 2):
                x_q = 1
            elif j < (WIDTH // 2):
                x_q = 0
            else:
                continue
            sums[y_q * 2 + x_q] += m[i][j]

    mult = reduce(lambda x, y: x * y, sums)


for i in range(10_000):
    for j in range(len(ps)):
        ps[j] = ((ps[j][0] + vs[j][0]) % WIDTH, (ps[j][1] + vs[j][1]) % HEIGHT)

    print("Step", i)
    draw(ps)
    print()
