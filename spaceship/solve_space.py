import argparse
from collections import deque

parser = argparse.ArgumentParser()
parser.add_argument("n", type=int)
parser.add_argument("--t", type=int, default=3)
args = parser.parse_args()

with open(f"problems/spaceship/{args.n:02d}.txt", "r") as f:
    d = f.read().strip()


targets = []
for line in d.split("\n"):
    v = line.split(" ")
    targets.append((int(v[0]), int(v[1])))


vecs = {
    (-1, -1): 1,
    (0, -1): 2,
    (1, -1): 3,
    (-1, 0): 4,
    (0, 0): 5,
    (1, 0): 6,
    (-1, 1): 7,
    (0, 1): 8,
    (1, 1): 9,
}


def solve(p, v, targets):
    q = deque()
    q.append((p, v, targets, []))
    visited = set()
    while len(q) > 0:
        p, v, targets, v_log = q.popleft()
        if len(targets) == 0:
            return v_log
        key = (p, v, len(targets))
        if key in visited:
            continue
        visited.add(key)
        t = targets[0]
        for vec in vecs.keys():
            new_v = v[0] + vec[0], v[1] + vec[1]
            new_p = p[0] + new_v[0], p[1] + new_v[1]
            new_v_log = v_log + [vec]
            if t == new_p:
                q.append((new_p, new_v, targets[1:], new_v_log))
            else:
                q.append((new_p, new_v, targets, new_v_log))


def move(p, v, v_log, targets):
    for i, vec in enumerate(v_log):
        t = targets[0]
        v = v[0] + vec[0], v[1] + vec[1]
        p = p[0] + v[0], p[1] + v[1]
        if p == t:
            if len(targets) == 1:
                return i + 1, p, v
            else:
                targets = targets[1:]


p = (0, 0)
v = (0, 0)
ans = []
# targets = targets[:10]
while len(targets) > 0:
    # 3 target を踏める最適を探索
    v_log = solve(p, v, targets[: args.t])
    # そのログから 1 target 分だけ move
    n, p, v = move(p, v, v_log, targets[:1])
    print(p, v)
    ans.extend(v_log[:n])
    targets = targets[1:]

print("".join([str(vecs[v]) for v in ans]))
