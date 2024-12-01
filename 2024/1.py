l, r = [], []
with open('1.txt') as infile:
    for row in infile.readlines():
        a, b = row.strip().split('  ')
        l.append(int(a))
        r.append(int(b))

l.sort()
r.sort()

res_1 = 0
res_2 = 0
for a, b in list(zip(l, r)):
    res_1 += abs(a - b)
    res_2 += a * r.count(a)

print(res_1, res_2)
