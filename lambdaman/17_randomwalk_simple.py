import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

MULT = 655608368
# MULT = 1593532120
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


X0 = 132813390
# X0 = 62500001
XT = X0
for i in range(500000):
    XT = (XT * MULT) % MOD
print(XT)
FA = (f, r, n)

randomwalk = fundef(
    (f, r),
    If(
        r == Int(XT),
        Str(""),
        Str("RDLU").drop(r % Int(4)).take(Int(1)).concat(f(r * X % M)),
    ),
)
randomwalk = fix1(randomwalk)

args = [Int((X0 * MULT) % MOD)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman16 ").concat(res)
print(ans)
