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
A = 1061
B = 84

r = Var("r")
dirs = Var("D")
FA = (f, dirs, r, n)

randomwalk = fundef(
    FA,
    If(
        n == Int(9000),
        Str(""),
        plus(
            FA,
            954944 - 4 + 1,
            954944,
            "DDUU",
            plus(
                FA,
                954820 - 8 + 1,
                954820,
                "UURRLLDD",
                plus(
                    FA,
                    954408 - 4 + 1,
                    954408,
                    "LLRR",
                    plus(
                        FA,
                        804842 - 4 + 1,
                        804842,
                        "UUDD",
                        plus(
                            FA,
                            804836 - 4 + 1,
                            804836,
                            "RRLL",
                            plus(
                                FA,
                                804830 - 4 + 1,
                                804830,
                                "RRLL",
                                plus(
                                    FA,
                                    802578 - 4 + 1,
                                    802578,
                                    "UUDD",
                                    dirs.drop(r / Int(1000) % Int(4) * Int(2))
                                    .take(Int(2))
                                    .concat(
                                        f(dirs)((r * Int(BASE_A + A) + Int(B)) % Int(2**32))(
                                            n - Int(2)
                                        )
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
)
randomwalk = fix(randomwalk)

args = [Str("RRDDLLUU"), Int(BASE_A + A + B), Int(1000000)]
res = randomwalk
for a in args:
    res = res(a)

# print(repr(res))
ans = Str("solve lambdaman13 ").concat(res)
print(ans)
