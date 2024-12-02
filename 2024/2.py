from copy import deepcopy

with open("2.txt") as infile:
    data = [[int(el) for el in row.strip().split(" ")] for row in infile.readlines()]

res1 = 0
res2 = 0
for row in data:
    bad = False

    if row != sorted(row) and row != sorted(row)[::-1]:
        bad = True

    if not bad:
        for el1, el2 in zip(row, row[1:]):
            if el1 == el2:
                bad = True
                break
            if abs(el1 - el2) > 3:
                bad = True
                break
    if not bad:
        res1 += 1

    if bad:
        print("fixing", row)
        variations = []
        for idx, val in enumerate(row):
            orig = deepcopy(row)
            orig.pop(idx)
            variations.append(orig)

        for variation in variations:
            bad = False
            if variation != sorted(variation) and variation != sorted(variation)[::-1]:
                bad = True

            if not bad:
                for el1, el2 in zip(variation, variation[1:]):
                    if el1 == el2:
                        bad = True
                        break
                    if abs(el1 - el2) > 3:
                        bad = True
                        break
            if not bad:
                break

    if not bad:
        res2 += 1

    word = "safe" if not bad else "unsafe"
    print(word, row)

print("1:", res1)
print("2:", res2)
