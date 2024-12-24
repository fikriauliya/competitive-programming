from itertools import combinations
from collections import defaultdict


conns = defaultdict(set)

while True:
    try:
        pair = input().split("-")
        conns[pair[0]].add(pair[1])
        conns[pair[1]].add(pair[0])

    except EOFError:
        break


def are_all_connected(nodes):
    for i in range(0, len(nodes)):
        for j in range(i + 1, len(nodes)):
            if nodes[j] not in conns[nodes[i]]:
                return False
    return True


keys = list(conns.keys())

res = []


def rec(nodes, i):
    global res
    if len(nodes) > len(res):
        res = nodes

    for j in range(i, len(keys)):
        if are_all_connected(nodes + [keys[j]]):
            rec(nodes + [keys[j]], j + 1)


rec([], 0)
print(",".join(sorted(res)))
