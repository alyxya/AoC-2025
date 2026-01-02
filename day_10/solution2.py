import sys
import cvxpy as cp

lines = []
with open(sys.argv[1]) as f:
    for line in f:
        lines.append(line.strip())

part2 = 0

for line in lines:
    els = line.split(" ")
    config = els[0][1:-1]
    joltage = [int(x) for x in els[-1][1:-1].split(",")]
    buttons = [[int(x) for x in el[1:-1].split(",")] for el in els[1:-1]]

    vars = [cp.Variable(integer=True) for _ in range(len(buttons))]
    constraints = []
    for pos in range(len(joltage)):
        lhs = 0
        for i in range(len(buttons)):
            button = buttons[i]
            for val in button:
                if val == pos:
                    lhs += vars[i]
        constraints.append(lhs == joltage[pos])
    for var in vars:
        constraints.append(var >= 0)

    total = 0
    for var in vars:
        total += var

    objective = cp.Minimize(total)

    prob = cp.Problem(objective, constraints)
    result = prob.solve()

    part2 += sum([v.value for v in vars])

    print(f"{part2=}")
print(f"{part2=}")
