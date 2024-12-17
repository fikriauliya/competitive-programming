m: list[list[str]] = []

while True:
    chs = list(input())
    if len(chs) == 0:
        break
    mapping = {"#": "##", "@": "@.", "O": "[]", ".": ".."}
    chs = list("".join(map(lambda x: mapping[x], chs)))
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


def can_move(py, px, c):
    mc = {">": (0, 1), "<": (0, -1), "v": (1, 0), "^": (-1, 0)}
    mov = mc[c]

    npy = py + mov[0]
    npx = px + mov[1]

    if m[npy][npx] == "#":
        return False

    if m[npy][npx] == ".":
        return True

    if abs(mov[1]) == 1:
        # horizontal
        return can_move(npy, npx, c)

    # vertical
    if m[npy][npx] == "[":
        return can_move(npy, npx, c) and can_move(npy, npx + 1, c)

    if m[npy][npx] == "]":
        return can_move(npy, npx, c) and can_move(npy, npx - 1, c)

    return False


def do_move(py, px, c):
    mc = {">": (0, 1), "<": (0, -1), "v": (1, 0), "^": (-1, 0)}
    mov = mc[c]

    npy = py + mov[0]
    npx = px + mov[1]

    if m[npy][npx] == "#":
        return

    if m[npy][npx] == ".":
        m[npy][npx] = m[py][px]
        m[py][px] = "."
        return

    if abs(mov[1]) == 1:
        # horizontal
        do_move(npy, npx, c)
        m[npy][npx] = m[py][px]
        m[py][px] = "."
        return

    if m[npy][npx] == "[":
        do_move(npy, npx, c)
        do_move(npy, npx + 1, c)
        m[npy][npx] = m[py][px]
        m[npy][npx + 1] = "."
        m[py][px] = "."
        return

    if m[npy][npx] == "]":
        do_move(npy, npx, c)
        do_move(npy, npx - 1, c)
        m[npy][npx] = m[py][px]
        m[npy][npx - 1] = "."
        m[py][px] = "."
        return


def move(py, px, c):
    if can_move(py, px, c):
        mc = {">": (0, 1), "<": (0, -1), "v": (1, 0), "^": (-1, 0)}
        mov = mc[c]
        do_move(py, px, c)

        return (py + mov[0], px + mov[1])
    else:
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
draw()

total = 0
for i in range(len(m)):
    for j in range(len(m[0])):
        if m[i][j] == "[":
            total += 100 * i + j

print(total)
