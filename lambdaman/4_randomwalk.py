import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

x = Var("x")
f = Var("f")
fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))

r = Var("r")
n = Var("n")
randomwalk = fundef(
    (f, r, n),
    If(
        n == Int(0),
        Str(""),
        Str("RDLU")
        .drop(r % Int(4))
        .take(Int(1))
        .concat(f((r * Int(48271)) % Int(2**31 - 1))(n - Int(1))),
    ),
)

randomwalk = fix(randomwalk)

args = [Int(1), Int(500000)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman4 ").concat(res)
print(ans)
