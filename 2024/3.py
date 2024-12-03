import re

with open("3.txt", "r") as infile:
    data = "".join(infile.readlines())


res1 = 0
for mult in re.findall(r"mul\((\d+),(\d+)\)", data):
    x, y = mult
    res1 += int(x) * int(y)

print(res1)

data = data.replace("don't()", "\n\ndon't()")
data = data.replace("do()", "\n\ndo()")

enabled = True
res2 = 0
for line in data.split("\n"):
    if line.startswith("don't()"):
        enabled = False

    if line.startswith("do()"):
        enabled = True

    if enabled:
        for mult in re.findall(r"mul\((\d+),(\d+)\)", line):
            x, y = mult
            res2 += int(x) * int(y)

print(res2)
