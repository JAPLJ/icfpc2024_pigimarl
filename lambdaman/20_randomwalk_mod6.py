import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

A = Int(1747473)
B = Int(50)
M = Int(4294967296)

f = Var("f")
r = Var("r")
n = Var("n")
x = Var("x")
step = Var("t")

fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))


def plus(c, fr, to, path, otherwise):
    path = Str(path)
    f, r, step, n = c
    return If(
        (Int(fr - 1) < n) & (n < Int(to + 1)),
        path.drop(n - Int(fr)).take(Int(1)).concat(f(r)(step)(n - Int(1))),
        otherwise,
    )


FA = (f, r, step, n)

randomwalk = fundef(
    FA,
    If(
        n == Int(0),
        Str(""),
        plus(
            FA,
            831001,
            831002,
            "LR",
            plus(
                FA,
                756302,
                756305,
                "RRLL",
                plus(
                    FA,
                    752648,
                    752651,
                    "LDUR",
                    plus(
                        FA,
                        381084,
                        381087,
                        "RUDL",
                        plus(
                            FA,
                            363380,
                            363381,
                            "UD",
                            plus(
                                FA,
                                351248,
                                351251,
                                "RUDL",
                                plus(
                                    FA,
                                    330544,
                                    330545,
                                    "UD",
                                    Str("RDLU")
                                    .drop(r / Int(1000) % Int(4))
                                    .take(Int(1))
                                    .concat(
                                        f(
                                            If(
                                                (step == Int(0))
                                                | (step == Int(2))
                                                | (step == Int(5)),
                                                (r * A + B) % M,
                                                r,
                                            )
                                        )((step + Int(1)) % Int(6))(n - Int(1))
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

args = [Int(1747523), Int(0), Int(1000000)]
res = randomwalk
for a in args:
    res = res(a)

ans = Str("solve lambdaman20 ").concat(res)
print(ans)
