import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

x = Var("x")
f = Var("f")
fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))

z = Var("z")
n = Var("n")
str_nth = fundef((z, n), z.drop(n).take(Int(1)))


def plus(c, fr, to, path, otherwise):
    path = Str(path)
    f, nth, dirs, r, n = c
    return If(
        (Int(fr - 1) < n) & (n < Int(to + 1)),
        nth(path)(n - Int(fr)).concat(f(nth)(dirs)(r)(n - Int(1))),
        otherwise,
    )


nth = Var("a")
r = Var("r")
dirs = Var("D")
FA = (f, nth, dirs, r, n)

randomwalk = fundef(
    FA,
    If(
        n == Int(200000),
        Str(""),
        plus(
            FA,
            871350,
            871351,
            "LR",
            plus(
                FA,
                843839,
                843850,
                "RUURRRLLLDDL",
                plus(
                    FA,
                    695438,
                    695443,
                    "UUUDDD",
                    plus(
                        FA,
                        663469,
                        663472,
                        "LLRR",
                        plus(
                            FA,
                            520799,
                            520802,
                            "UUDD",
                            plus(
                                FA,
                                520768,
                                520769,
                                "DU",
                                plus(
                                    FA,
                                    412139,
                                    412140,
                                    "UD",
                                    plus(
                                        FA,
                                        380044,
                                        380053,
                                        "LLLLLRRRRR",
                                        plus(
                                            FA,
                                            220519,
                                            220528,
                                            "LLLLLRRRRR",
                                            nth(dirs)(r / Int(1000) % Int(4)).concat(
                                                f(nth)(dirs)(
                                                    (r * Int(1664524 + 449) + Int(18)) % Int(2**32)
                                                )(n - Int(1))
                                            ),
                                        ),
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

args = [str_nth, Str("RDLU"), Int(1664991), Int(1000000)]
res = randomwalk
for a in args:
    res = res(a)

# print(repr(res))
ans = Str("solve lambdaman18 ").concat(res)
print(ans)
