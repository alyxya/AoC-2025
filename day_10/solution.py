import sys

lines = []
with open(sys.argv[1]) as f:
    for line in f:
        lines.append(line.strip())

part1 = 0

for line in lines:
    els = line.split(" ")
    config = els[0][1:-1]
    joltage = [int(x) for x in els[-1][1:-1].split(",")]
    buttons = [[int(x) for x in el[1:-1].split(",")] for el in els[1:-1]]

    best = float('inf')
    for i in range(1 << len(buttons)):
        cur = 0
        state = ["." for _ in range(len(config))]
        for j in range(len(buttons)):
            if i & (1 << j):
                cur += 1
                for val in buttons[j]:
                    state[val] = "." if state[val] == "#" else "#"
        if "".join(state) == config:
            best = min(best, cur)
    part1 += best
print(f"{part1=}")
