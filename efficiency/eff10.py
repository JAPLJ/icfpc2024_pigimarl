import sys
from z3 import *


sys.setrecursionlimit(10**6)

all_pairs = []


class Cond:
    def __init__(self, v):
        if isinstance(v, str):
            self.v = Int(v)
        else:
            self.v = v

    def __and__(self, other):
        return Cond(And(self.v, other.v))

    def __or__(self, other):
        return Cond(Or(self.v, other.v))

    def __invert__(self):
        return Cond(Not(self.v))

    def __eq__(self, other):
        if isinstance(other, int):
            all_pairs.append(self.v == other)
            print(self.v, other)
            return Cond(self.v == other)
        else:
            print(self.v, other.v)
            all_pairs.append(Not(self.v == other.v))
        return Cond(self.v == other.v)


conds_19 = []
for vn in range(5, 86):
    exec(f'v{vn} = Cond(f"v{vn}")')
    # if vn <= 13:
    #     conds_19.append(eval(f"v{vn}.v == {vn-4}"))
    # else:
    conds_19.append(eval(f"1 <= v{vn}.v"))
    conds_19.append(eval(f"v{vn}.v <= 9"))


def skip_ws(s, p):
    while p < len(s) and s[p] == " ":
        p += 1
    return p


def parse_term(s, p):
    p = skip_ws(s, p)
    if s[p] == "(":
        e, p = parse(s, p + 1)
        p = skip_ws(s, p)
        assert s[p] == ")"
        return e, p + 1
    elif s[p] == "~":
        e, p = parse(s, p + 1)
        p = skip_ws(s, p)
        return ~e, p
    elif s[p] == "v" or s[p].isdigit():
        num = 0
        var = s[p] == "v"
        if var:
            p += 1
        while s[p].isdigit():
            num = num * 10 + int(s[p])
            p += 1
        p = skip_ws(s, p)
        if var:
            return eval(f"v{num}"), p
        else:
            return num, p


def parse(s, p):
    p = skip_ws(s, p)
    e, p = parse_term(s, p)
    p = skip_ws(s, p)
    while p < len(s) and s[p] in "&|=":
        op = s[p]
        e2, p = parse_term(s, p + 1)
        if op == "&":
            e = e & e2
        elif op == "|":
            e = e | e2
        else:
            e = e == e2
        p = skip_ws(s, p)
    return e, p


with open("eff11.expr", "r") as f:
    e = parse(f.read(), 0)[0]

p = Solver()
p.add(conds_19)
p.add(all_pairs)
print("checking...")
assert p.check() == sat

for vn in range(5, 86):
    print(f"*** v{vn} ***")
    for val in range(1, 10):
        p.push()
        p.add(eval(f"v{vn}.v == {val}"))
        if not p.check() == sat:
            p.pop()
        else:
            print(vn, val)
            break


if p.check() == sat:
    m = p.model()
    ans = 0
    for d in m.decls():
        num = int(str(d)[1:])
        num = 85 - num
        ans += 9**num * (m[d].as_long() - 1)
    print(ans)
else:
    print("unsat")
