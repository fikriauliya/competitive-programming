import random
t = int(input())
for tt in range(t):
    n = int(input())

    res = 0
    trials = 1000
    for i in range(trials):
        arr = [i for i in range(1, n+1)]
        random.shuffle(arr)
        last = None

        cards = 0
        while len(arr) > 0:
            top = arr.pop()
            if last == None:
                last = top
                cards += 1
            else:
                if last < top:
                    last = top
                    cards += 1
        res += cards
    res = res/trials
    print("Case #{}: {}\n".format(tt+1, res))
