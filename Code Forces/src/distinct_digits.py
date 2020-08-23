l, r= [int(i) for i in str.split(input())]

def diff(n):
    return len(list(n)) == len(set(n))


def f():
    for n in range(l, r+1):
        if diff(str(n)):
            print(n)
            return

    print(-1)

f()

