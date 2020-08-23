import itertools

def f():
    d = [int(i) for i in str.split(input())]
    for i in range(1, 4):
        l = list(itertools.combinations(range(4), i))
        for ll in l:
            a, b = [0, 0]
            a = sum([d[i] for i in range(4) if i in ll])
            b = sum([d[i] for i in range(4) if i not in ll])
            if a == b:
                print("YES")
                return

    print("NO")

f()
