import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

x = Var("x")
f = Var("f")
fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))

n = Var("n")


def plus(c, fr, to, path, otherwise):
    path = Str(path)
    f, dirs, r, n = c
    return If(
        (Int(fr - 1) < n) & (n < Int(to + 1)),
        path.drop(n - Int(fr)).take(Int(1)).concat(f(dirs)(r)(n - Int(1))),
        otherwise,
    )


BASE_A = 1664524
A = 27
B = 73

r = Var("r")
dirs = Var("D")
FA = (f, dirs, r, n)

randomwalk = fundef(
    FA,
    If(
        n == Int(16000),
        Str(""),
        plus(
            FA,
            635218 - 24 + 1,
            635218,
            "RRDDDDDDDDUUUUUUUURRLLLL",
            dirs.drop(r / Int(1000) % Int(4) * Int(2))
            .take(Int(2))
            .concat(f(dirs)((r * Int(BASE_A + A) + Int(B)) % Int(2**32))(n - Int(2))),
        ),
    ),
)
randomwalk = fix(randomwalk)

args = [Str("RRDDLLUU"), Int(BASE_A + A + B), Int(1000000)]
res = randomwalk
for a in args:
    res = res(a)

# print(repr(res))
ans = Str("solve lambdaman15 ").concat(res)
print(ans)
