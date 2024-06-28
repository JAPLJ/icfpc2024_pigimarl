from common.trans import *

x = Var("x")
f = Var("f")
fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))

d = Var("d")
n = Var("n")
straight = Lambda(f, Lambda(d, Lambda(n, If(n == Int(0), Str(""), d.concat(f(d)(n - Int(1)))))))
straight = fix(straight)

s = Var("s")
rot = Lambda(
    f,
    Lambda(
        s,
        Lambda(
            n,
            If(
                n == Int(0),
                Str(""),
                s(Str("R"))(n)
                .concat(s(Str("D"))(n))
                .concat(s(Str("L"))(n))
                .concat(s(Str("U"))(n))
                .concat(f(s)(n - Int(1))),
            ),
        ),
    ),
)
rot = fix(rot)

print(rot(straight)(Int(100)))
