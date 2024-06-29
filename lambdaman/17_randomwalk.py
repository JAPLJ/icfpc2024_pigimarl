import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

x = Var("x")
f = Var("f")
fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))

z = Var("z")
n = Var("n")
str_nth = fundef((z, n), z.drop(n).take(Int(1)))

nth = Var("a")
r = Var("r")
dirs = Var("D")
randomwalk = fundef(
    (f, nth, dirs, r, n),
    If(
        n == Int(0),
        Str(""),
        nth(dirs)(r / Int(1000) % Int(4)).concat(
            f(nth)(dirs)((r * Int(1664525) + Int(10)) % Int(2**32))(n - Int(1))
        ),
    ),
)
randomwalk = fix(randomwalk)

args = [str_nth, Str("RDLU"), Int(1), Int(500000)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman17 ").concat(res)
print(ans)
