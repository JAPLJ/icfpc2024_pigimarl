import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

A = Int(1732529)
B = Int(13)
M = Int(4294967296)

f = Var("f")
r = Var("r")
n = Var("n")
x = Var("x")

fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))


def plus(c, fr, to, path, otherwise):
    path = Str(path)
    f, r, n = c
    return If(
        (Int(fr - 1) < n) & (n < Int(to + 1)),
        path.drop(n - Int(fr)).take(Int(1)).concat(f(r)(n - Int(1))),
        otherwise,
    )


FA = (f, r, n)

randomwalk = fundef(
    FA,
    If(
        n == Int(0),
        Str(""),
        plus(
            FA,
            812962,
            812963,
            "UD",
            Str("RDLU")
            .drop(r / Int(1000) % Int(4))
            .take(Int(1))
            .concat(f((r * A + B) % M)(n - Int(1))),
        ),
    ),
)
randomwalk = fix(randomwalk)

args = [Int(1732542), Int(1000000)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman21 ").concat(res)
print(ans)
