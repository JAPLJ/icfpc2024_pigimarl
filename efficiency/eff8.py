from z3 import *


class Cond:
    def __init__(self, v):
        if isinstance(v, str):
            self.v = Bool(v)
        else:
            self.v = v

    def __and__(self, other):
        return Cond(And(self.v, other.v))

    def __or__(self, other):
        return Cond(Or(self.v, other.v))

    def __invert__(self):
        return Cond(Not(self.v))


for vn in range(5, 55):
    exec(f'v{vn} = Cond(f"v{vn}")')


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
    else:
        assert s[p] == "v"
        num = 0
        p += 1
        while s[p].isdigit():
            num = num * 10 + int(s[p])
            p += 1
        p = skip_ws(s, p)
        return eval(f"v{num}"), p


def parse(s, p):
    p = skip_ws(s, p)
    e, p = parse_term(s, p)
    p = skip_ws(s, p)
    while p < len(s) and s[p] in "&|":
        op = s[p]
        e2, p = parse_term(s, p + 1)
        if op == "&":
            e = e & e2
        else:
            e = e | e2
        p = skip_ws(s, p)
    return e, p


with open("eff8.expr", "r") as f:
    e = parse(f.read(), 0)[0]

p = Solver()
p.add(e.v)
assert p.check() == sat

cs = []
for vn in range(54, 4, -1):
    cd_f = eval(f"v{vn}.v == False")
    s = Solver()
    s.add(e.v)
    for c in cs:
        s.add(c)
    s.add(cd_f)
    if s.check() == sat:
        print(vn, False)
        cs.append(cd_f)
    else:
        print(vn, True)
        cd_t = eval(f"v{vn}.v == True")
        cs.append(cd_t)

s = Solver()
s.add(e.v)
for c in cs:
    s.add(c)

if s.check() == sat:
    m = s.model()
    ans = 0
    for d in m.decls():
        num = int(str(d)[1:])
        num = num - 5
        if m[d]:
            ans += 2**num
    print(ans)
else:
    print("unsat")
