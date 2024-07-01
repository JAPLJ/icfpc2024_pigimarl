import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

X = Int(48271)
M = Int(2147483647)

f = Var("f")
r = Var("r")
n = Var("n")
x = Var("x")

# fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))
fix = Lambda(f, Lambda(x, x(x))(Lambda(x, f(x(x)))))


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
    (f, r),
    If(
        r == Int(1695999298),
        Str(""),
        Str("RDLU").drop(r % Int(4)).take(Int(1)).concat(f(r * X % M)),
    ),
)
randomwalk = fix(randomwalk)

args = [Int(1)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman7 ").concat(res)
print(ans)
