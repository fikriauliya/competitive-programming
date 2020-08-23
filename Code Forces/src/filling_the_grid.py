h, w = [int(i) for i in str.split(input())]
r = [int(i) for i in str.split(input())]
c = [int(i) for i in str.split(input())]

m = [[None for i in range(w)] for j in range(h)]
def f():
    for rr in range(h):
        for cc in range(r[rr]):
            if m[rr][cc] == False:
                print(0)
                return
            m[rr][cc] = True
        if r[rr] < w:
            if m[rr][r[rr]] == True:
                print(0)
                return
            m[rr][r[rr]] = False
    for cc in range(w):
        for rr in range(c[cc]):
            if m[rr][cc] == False:
                print(0)
                return
            m[rr][cc] = True
        if c[cc] < h:
            if m[c[cc]][cc] == True:
                print(0)
                return
            m[c[cc]][cc] = False

    none = sum([1 for i in range(w) for j in range(h) if m[j][i] is None])
    print(pow(2, none, 1000000007))

f()
