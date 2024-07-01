import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

MULT = 490190852
MOD = 2147483647
X = Int(MULT)
M = Int(MOD)

f = Var("f")
r = Var("r")
n = Var("n")
x = Var("x")

# fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))
fix = Lambda(f, Lambda(x, x(x))(Lambda(x, f(x(x)))))


def fix1(f):
    return Lambda(x, x(x))(Lambda(x, f(x(x))))


def plus(c, fr, to, path, otherwise):
    path = Str(path)
    f, r, n = c
    return If(
        (Int(fr - 1) < n) & (n < Int(to + 1)),
        path.drop(n - Int(fr)).take(Int(1)).concat(f(r)(n - Int(1))),
        otherwise,
    )


X0 = 125002403
XT = X0
for i in range(480050):
    XT = (XT * MULT) % MOD

FA = (f, r, n)

randomwalk = fundef(
    (f, r),
    If(
        r == Int(XT),
        Str(""),
        Str("RRDDLLUU").drop(r % Int(4) * Int(2)).take(Int(2)).concat(f(r * X % M)),
    ),
)
randomwalk = fix1(randomwalk)

args = [Int((X0 * MULT) % MOD)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman11 ").concat(res)
print(ans)
