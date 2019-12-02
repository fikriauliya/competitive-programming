from collections import defaultdict

def sort(items):
    counts = defaultdict(lambda: 0)
    for item in items:
        counts[item] += 1

    res = [0] * len(items)
    ctr = 0
    for k in range(10):
        for j in range(counts[k]):
            res[ctr] = k
            ctr += 1
    return res

res = sort([4,1,3,6,4,6,1,3,4,6])
print(res)
