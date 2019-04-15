from typing import List
from collections import deque
from itertools import starmap


def calc_distance(m: List[List[int]]):
    locs = set([(r, c)
                for r in range(len(m))
                for c in range(len(m[0]))
                if m[r][c] == 1])
    distances = [[-1 for i in range(c)] for j in range(r)]
    for loc in locs:
        distances[loc[0]][loc[1]] = 0

    cur_distance = 0
    while locs:
        movements = [(-1, 0), (0, -1), (0, 1), (1, 0)]
        cur_distance += 1
        new_locs = set()
        for loc in locs:
            new_locs = new_locs.union(set(filter(
                lambda x: x[0] in range(0, r) and x[1] in range(
                    0, c) and distances[x[0]][x[1]] == -1,
                map(lambda x: (x[0] + loc[0], x[1] + loc[1]), movements)
            )))

        for new_loc in new_locs:
            distances[new_loc[0]][new_loc[1]] = cur_distance

        locs = new_locs

    return distances


def find_shortest(distances, min_distance, max_distance):
    mid_distance = (max_distance - min_distance) // 2
    locs_with_larger_distance = [(i, j) for i in range(r)
                                 for j in range(c) if distances[i][j] >= mid_distance]
    cur_distance = max([abs(locs[0] - i) + abs(locs[1] - j) for locs in locs_with_larger_distance
                        for i in range(r)
                        for j in range(c)])
    if (cur_distance == mid_distance):
        find_shortest(distances,
    if (max_distance > cur_distance):


t=int(input())
for tt in range(t):
    r, c=[int(i) for i in str.split(input(), ' ')]
    m=[]
    for rr in range(r):
        m.append([int(c) for c in input()])
    print('m', m)

    distances=calc_distance(m)
    print('distances', distances)

    max_distance=max(distances)
    find_shortest(distances, (max_distance//2))
