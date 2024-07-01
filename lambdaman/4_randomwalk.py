# 4, 5, 7, 10
import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

x = Var("x")
f = Var("f")
fix = Lambda(f, Lambda(x, x(x))(Lambda(x, f(x(x)))))


def fix1(f):
    return Lambda(x, x(x))(Lambda(x, f(x(x))))


r = Var("r")
n = Var("n")
randomwalk = fundef(
    (f, r),
    If(
        r == Int(1450551721),
        Str(""),
        Str("RDLU").drop(r % Int(4)).take(Int(1)).concat(f((r * Int(48271)) % Int(2**31 - 1))),
    ),
)

randomwalk = fix1(randomwalk)

args = [Int(1)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman4 ").concat(res)
print(ans)
