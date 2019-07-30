class Node:
    def __init__(self, name):
        self._edges = []
        self._name = name

    def connect(self, node):
        self._edges.append(node)

    def to_s(self):
        return "{} : {}".format(self._name, len(self._edges))


def find_root(visited, node: Node):
    if len(node._edges) == 1:
        return node

    for edge in node._edges:
        if edge not in visited:
            visited[edge] = True
            return find_root(visited, edge)


n = int(input())
nodes = {}
root = None
for nn in range(n-1):
    v1, v2, c = map(int, input().split())

    if v1 in nodes:
        v1 = nodes[v1]
    else:
        nodes[v1] = Node(v1)
        v1 = nodes[v1]
    if v2 in nodes:
        v2 = nodes[v2]
    else:
        nodes[v2] = Node(v2)
        v2 = nodes[v2]
    v1.connect(v2)
    v2.connect(v1)

    if not root:
        root = v1

visited = {}
root = find_root(visited, root)
dfs(root)
