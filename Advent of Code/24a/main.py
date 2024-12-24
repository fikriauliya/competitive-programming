import re
from dataclasses import dataclass

vars = {}


@dataclass
class Or:
    a: str
    b: str

    def eval(self):
        return vars[self.a].eval() | vars[self.b].eval()


@dataclass
class And:
    a: str
    b: str

    def eval(self):
        return vars[self.a].eval() & vars[self.b].eval()


@dataclass
class Xor:
    a: str
    b: str

    def eval(self):
        return vars[self.a].eval() ^ vars[self.b].eval()


@dataclass
class Num:
    a: int

    def eval(self):
        return self.a


while True:
    line = input()
    if len(line) == 0:
        break

    a, b = line.split(": ")
    vars[a] = Num(int(b))

while True:
    try:
        s = input()
        pattern = r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)"
        match = re.match(pattern, s)
        if match:
            a, op, b, c = match.groups()
            if op == "AND":
                vars[c] = And(a, b)
            elif op == "OR":
                vars[c] = Or(a, b)
            elif op == "XOR":
                vars[c] = Xor(a, b)
    except EOFError:
        break

print(vars)

res = 0
for var in vars.keys():
    pattern = r"z(\d+)"
    match = re.match(pattern, var)
    if match:
        num = int(match.groups()[0])
        cur_res = vars[var].eval()
        res = res | cur_res << num

print(res)
