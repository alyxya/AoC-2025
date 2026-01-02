import sys

d = {}

with open(sys.argv[1]) as f:
    for line in f:
        line = line.strip()
        source, dests = line.split(": ")
        dests = dests.split(" ")
        d[source] = dests

dp = {"out": 1}

def solve(cur: str):
    if cur in dp:
        return dp[cur]
    nexts = d[cur]
    ans = 0
    for next in nexts:
        ans += solve(next)
    dp[cur] = ans
    return dp[cur]

print(solve("you"))
