import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

MULT = 1303812212
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


X0 = 531252074
# X0 = 62500001
XT = X0
z = set()
z.add(X0)
detected = False
for i in range(500000):
    XT = (XT * MULT) % MOD
    if XT in z and not detected:
        print("collision")
        detected = True
    z.add(XT)

FA = (f, r, n)

randomwalk = fundef(
    (f, r, n),
    If(
        n == Int(0),
        Str(""),
        Str("RRDDLLUU").drop(r % Int(4) * Int(2)).take(Int(2)).concat(f(r * X % M)(n - Int(1))),
    ),
)
randomwalk = fix1(randomwalk)

args = [Int((X0 * MULT) % MOD), Int(500000)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman16 ").concat(res)
print(ans)
