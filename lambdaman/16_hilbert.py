import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

D = Var("D")

x = Var("x")
f = Var("f")
fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))

d = Var("d")
n = Var("n")
straight = fundef((f, d, n), If(n == Int(0), Str(""), d.concat(f(d)(n - Int(1)))))
straight = fix(straight)

z = Var("z")
str_nth = fundef((z, n), z.drop(n).take(Int(1)))

steps = Int(128)
L = Var("L")

p = Var("p")
sgn = Var("S")
go = Var("s")
nth = Var("t")
hilbert = fundef(
    (f, D, L, go, nth, p, sgn, n),
    If(
        n == Int(0),
        go(nth(D)(p))(L).concat(go(nth(D)(p + Int(1)))(L)).concat(go(nth(D)(p + Int(2)))(L)),
        f(D)(L)(go)(nth)((p + sgn) % Int(12))(Int(12) - sgn)(n - Int(1))
        .concat(f(D)(L)(go)(nth)(p)(sgn)(n - Int(1)))
        .concat(go(nth(D)(p + Int(1)))(L))
        .concat(f(D)(L)(go)(nth)(p)(sgn)(n - Int(1)))
        .concat(f(D)(L)(go)(nth)((p + sgn * Int(2)) % Int(12))(sgn)(n - Int(1))),
    ),
)
hilbert = fix(hilbert)

args = [
    Str("DRURDLLURULD"),
    Int(128),
    straight,
    str_nth,
    Int(0),
    Int(3),
    Int(5),
]
res = hilbert
for a in args:
    res = res(a)

ans = Str("solve lambdaman16 ").concat(res)
print(ans)
