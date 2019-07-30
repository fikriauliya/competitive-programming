import functools


def solve():
    row, col = [int(i) for i in str.split(input(), ' ')]
    m = []
    for i in range(row):
        m.append(list(map(int, str.split(input(), ' '))))

    total = functools.reduce(lambda x, y: x ^ y, list(map(lambda x: x[0], m)))

    if total == 0:
        for i in range(row):
            for j in range(1, col):
                if m[i][j] != m[i][0]:
                    print('TAK\n' +
                          ' '.join(['1'] * i + [str(j + 1)] + ['1'] * (row - i - 1)))
                    return
        print("NIE")
    else:
        print("TAK\n" + ' '.join(['1'] * row))


solve()
