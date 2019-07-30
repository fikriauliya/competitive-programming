n = int(input())
m = []

for i in range(n):
    a, b = map(int, str.split(input(), ' '))
    m.append(((a - b), a, b))

m.sort(reverse=True)
score = 0
for i in range(n):
    _, a, b = m[i]
    score += (i + 1) * (a - b) + b * n - a

print(score)
