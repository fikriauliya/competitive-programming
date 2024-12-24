from collections import defaultdict


conns = defaultdict(set)

while True:
    try:
        pair = input().split("-")
        conns[pair[0]].add(pair[1])
        conns[pair[1]].add(pair[0])

    except EOFError:
        break

res = set()
for k, v in conns.items():
    adjs1 = conns[k]
    for adj in adjs1:
        if adj > k:
            adjs2 = conns[adj]
            intersections = adjs1.intersection(adjs2)
            for intersection in intersections:
                if (
                    k.startswith("t")
                    or adj.startswith("t")
                    or intersection.startswith("t")
                ):
                    res.add("-".join(sorted([k, adj, intersection])))

print(len(res))
