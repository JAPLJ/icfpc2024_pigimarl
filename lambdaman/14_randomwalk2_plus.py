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
A = 433
B = 10

r = Var("r")
dirs = Var("D")
FA = (f, dirs, r, n)

randomwalk = fundef(
    FA,
    If(
        n == Int(8000),
        Str(""),
        plus(
            FA,
            926662 - 8 + 1,
            926662,
            "UUUUDDDD",
            plus(
                FA,
                835400 - 8 + 1,
                835400,
                "UULLRRDD",
                plus(
                    FA,
                    722862 - 8 + 1,
                    722862,
                    "DDUURRLL",
                    plus(
                        FA,
                        664218 - 4 + 1,
                        664218,
                        "LLRR",
                        plus(
                            FA,
                            249024 - 8 + 1,
                            249024,
                            "UURRLLDD",
                            plus(
                                FA,
                                248902 - 12 + 1,
                                248902,
                                "UURRDDUULLDD",
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
)
randomwalk = fix(randomwalk)

args = [Str("RRDDLLUU"), Int(BASE_A + A + B), Int(1000000)]
res = randomwalk
for a in args:
    res = res(a)

# print(repr(res))
ans = Str("solve lambdaman14 ").concat(res)
print(ans)
