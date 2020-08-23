t=int(input())
for tt in range(t):
    inp = input()
    if len(set(inp)) == 1:
        print(inp)
    else:
        print(len(inp) * "01")
