# debug = print
debug = lambda *args, **kwargs: None


def draw(m, a, b):
    for i in range(len(m)):
        debug('  ' * (len(m) - i - 1), end='')
        for j in range(len(m[i])):
            if i == a[0] and j == a[1]:
                debug('a', end=' ')
            elif i == b[0] and j == b[1]:
                debug('b', end=' ')
            else:
                debug(m[i][j], end=' ')

        debug()
    debug()


def next_moves(pos, m):
    res = []
    if pos[1] > 0:
        res.append((pos[0], pos[1] - 1))
    if pos[1] < (pos[0] + 1) * 2 - 2:
        res.append((pos[0], pos[1] + 1))
    if pos[1] % 2 == 1 and pos[0] > 0:
        res.append((pos[0]-1, pos[1]-1))
    elif pos[0] < len(m) - 1:
        res.append((pos[0]+1, pos[1]+1))

    res = list(filter(lambda p: m[p[0]][p[1]] == 0, res))
    return res


def rec(m, a, b, total_a, total_b, turn='a'):
    debug(turn, a) if turn == 'a' else debug(turn, b)

    if len(next_moves(a, m)) == 0 and len(next_moves(b, m)) == 0:
        score = total_a - total_b
        debug(score, total_a, total_b)
        # draw(m, a, b)
        return score

    nexts = next_moves(a, m) if turn == 'a' else next_moves(b, m)
    if len(nexts) == 0:
        score = rec(m, a, b, total_a, total_b,
                    turn=('b' if turn == 'a' else 'a'))
    else:
        score = None
        for next in nexts:
            m[next[0]][next[1]] = 1
            if turn == 'a':
                new_score = rec(m, next, b, total_a + 1, total_b, turn='b')
            else:
                new_score = rec(m, a, next, total_a, total_b + 1, turn='a')
            score = new_score if score is None else max(score, new_score)
            m[next[0]][next[1]] = 0
    return score


t = int(input())
for tt in range(t):
    s, ra, pa, rb, pb, c = list(map(int, input().split(' ')))
    m = [[0 for i in range(2 * j + 1)] for j in range(s)]
    for cc in range(c):
        r, p = list(map(int, input().split(' ')))
        m[r-1][p-1] = 1

    m[ra-1][pa-1] = 1
    m[rb-1][pb-1] = 1

    draw(m, (ra-1, pa-1), (rb-1, pb-1))
    res = rec(m, (ra-1, pa-1), (rb-1, pb-1), 1, 1, 'a')
    print("Case #{}: {}".format(tt+1, res))
