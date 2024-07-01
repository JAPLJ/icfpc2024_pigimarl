import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

f = Var("f")
n = Var("n")
x = Var("x")

# fix = Lambda(f, Lambda(x, f(x(x)))(Lambda(x, f(x(x)))))
fix = Lambda(f, Lambda(x, x(x))(Lambda(x, f(x(x)))))


def fix1(f):
    return Lambda(x, x(x))(Lambda(x, f(x(x))))


s = Var("s")
t = Var("t")
cat = fundef((s,), s.concat(s).concat(s))

f = Var("f")
quad = fundef((f, s), f(f(f(f(s)))))

res = fundef((f,), f(f(Str("DD")).concat(f(Str("LL"))).concat(f(Str("UU"))).concat(f(Str("RR")))))(
    quad(cat)
)

ans = Str("solve lambdaman8 ").concat(res)
print(ans)
print(len(str(ans)), file=sys.stderr)
