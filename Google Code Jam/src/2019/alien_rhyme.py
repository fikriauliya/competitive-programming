res = 0


class Node:
    def __init__(self, level):
        self.children = {}
        self.value = None
        self.level = level

    def insert(self, value):
        node = self
        for c in value:
            if c not in node.children:
                node.children[c] = Node(self.level + 1)
            node = node.children[c]
        node.value = value

    def print(self):
        if self.value is not None:
            print(self.value)
        for k in self.children:
            print(">", end="")
            self.children[k].print()
            print("<", end="")

    def compact(self):
        node = self
        global res
        to_be_deleted_keys = set()
        for k, child in node.children.items():
            child.compact()
            if len(child.children) == 0 and child.value is None:
                # delete
                to_be_deleted_keys.add(k)
        for k in to_be_deleted_keys:
            del node.children[k]

        if len(node.children) == 1 and len(list(node.children.values())[0].children) == 0:
            if node.value:
                # print("+", node.value, list(node.children.values())[0].value)
                res += 2
                node.value = None
            else:
                node.value = list(node.children.values())[0].value
            node.children = {}

        if len(node.children) >= 2 and self.level > 0:
            l = list(node.children.values())
            if (l[0].value is not None and l[1].value is not None):
                # print("+", l[0].value, l[1].value)
                res += 2
                node.children = {}


t = int(input())  # read a line with a single integer
for i in range(1, t + 1):
    res = 0
    root = Node(0)
    n = int(input())
    ws = [input() for w in range(n)]
    for w in ws:
        root.insert(w[::-1])
    # root.print()
    root.compact()
    # root.print()
    print("Case #{}: {}".format(i, res))
