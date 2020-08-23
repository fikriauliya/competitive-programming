max_doms = 7
doms = [(i, j) for i in range(1,max_doms) for j in range(i,max_doms)]
d_ind = {i: set() for i in range(1, max_doms)}
for i, dom in enumerate(doms):
    d_ind[dom[0]].add(i)
    d_ind[dom[1]].add(i)
#i print(doms)
# print(d_ind)

n, m =  [int(i) for i in str.split(input())]
g = {i: [] for i in range(1, n + 1)}
edges = {}
for i in range(m):
    a, b =  [int(i) for i in str.split(input())]

    g[a].append(b)
    g[b].append(a)
    edges[(a ,b)] = False
    edges[(b, a)] = False

# print(g)

res = 0
unused_ind = set(range(0, len(doms)))
placed_num = {i: None for i in range(1, n + 1)}

def dfs(v1, unused_ind, placed_num, ctr):
    global res
    global m
    print(ctr, 'v1', v1, 'unused_ind', unused_ind, 'placed_num', placed_num)
    if ctr == m:
        res = m
        return
    if ctr > res:
        res = ctr

    if not placed_num[v1]:
        c_unused_index = unused_ind
        poss_v1_num = set(range(1, max_doms))
        for v2 in g[v1]:
            if placed_num[v2]:
                c_unused_index = d_ind[placed_num[v2]] & c_unused_index
                for cu in c_unused_index:
                    if doms[cu][0] == placed_num[v2]:
                        poss_v1_num = poss_v1_num & {doms[cu][1]}
                    else:
                        poss_v1_num = poss_v1_num & {doms[cu][0]}
    else:
        poss_v1_num = set([placed_num[v1]])

    for p_v1_num in poss_v1_num:
        for v2 in g[v1]:
            if edges[(v1, v2)] == False:
                poss_index = d_ind[p_v1_num] & unused_ind
                for p_ind in poss_index:
                    if doms[p_ind][0] == p_v1_num:
                        p_v2_num = doms[p_ind][1]
                    else:
                        p_v2_num = doms[p_ind][0]

                    placed_num[v1] = p_v1_num
                    placed_num[v2] = p_v2_num
                    edges[(v1, v2)] = True
                    edges[(v2, v1)] = True

                    dfs(v2, unused_ind - {p_ind}, placed_num, ctr + 1)

                    placed_num[v1] = None
                    placed_num[v2] = None
                    edges[(v1, v2)] = False
                    edges[(v2, v1)] = False

dfs(1, unused_ind, placed_num, 0)
print(res)

