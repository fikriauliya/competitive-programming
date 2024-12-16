m: list[list[str]] = []

while True:
    chs = list(input())
    if len(chs) == 0:
        break
    m.append(chs)

py = 0
px = 0

for i in range(len(m)):
    for j in range(len(m[0])):
        if m[i][j] == "@":
            py = i
            px = j
            break


def draw():
    for i in range(len(m)):
        for j in range(len(m[0])):
            print(m[i][j], end="")
        print()


def move(py, px, c):
    mc = {">": (0, 1), "<": (0, -1), "v": (1, 0), "^": (-1, 0)}
    mov = mc[c]

    npy = py + mov[0]
    npx = px + mov[1]

    if m[npy][npx] == "#":
        return (py, px)

    if m[npy][npx] == ".":
        m[npy][npx] = m[py][px]
        m[py][px] = "."
        return (npy, npx)

    if m[npy][npx] == "O":
        nnpy, nnpx = move(npy, npx, c)
        if nnpy != npy or nnpx != npx:
            m[npy][npx] = m[py][px]
            m[py][px] = "."
            return (npy, npx)
        else:
            return (py, px)

    return (py, px)


movements = ""
while True:
    try:
        movements += input()
    except EOFError:
        break

draw()

for movement in movements:
    py, px = move(py, px, movement)

print()
draw()

total = 0
for i in range(len(m)):
    for j in range(len(m[0])):
        if m[i][j] == "O":
            total += 100 * i + j

print(total)
