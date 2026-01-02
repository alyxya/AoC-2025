import sys

d = {}

with open(sys.argv[1]) as f:
    for line in f:
        line = line.strip()
        source, dests = line.split(": ")
        dests = dests.split(" ")
        d[source] = dests

dp = {("out", 3): 1}

def solve(cur: str, state: int):
    if (cur, state) in dp:
        return dp[(cur, state)]
    if cur not in d:
        return 0
    nexts = d[cur]
    ans = 0
    for next in nexts:
        ans += solve(next, state | (1 if next == "dac" else 0) | (2 if next == "fft" else 0))
    dp[(cur, state)] = ans
    return dp[(cur, state)]

print(solve("svr", 0))
