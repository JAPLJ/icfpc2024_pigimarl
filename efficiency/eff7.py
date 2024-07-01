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


for vn in range(5, 45):
    exec(f'v{vn} = Cond(f"v{vn}")')


with open("eff7.expr", "r") as f:
    e = eval(f.read())

s = Solver()
s.add(e.v)

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
