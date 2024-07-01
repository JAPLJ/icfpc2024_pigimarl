import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *


cat = Var("C")
s = Var("s")
cat_fn = fundef((s,), s.concat(s))

rep = Var("R")
f = Var("f")
x = Var("x")
rep_fn = fundef((f, x), f(f(f(x))))

rep100 = Var("Q")
# rep100_fn = fundef((rep,), rep(rep(cat_fn)))(rep_fn)
applytwice = fundef((f, x), f(f(x)))
z = Var("z")
apply256times = fundef((z,), z(z))(fundef((z,), z(z))(applytwice))
a = Var("a")
rep100_fn = fundef((s,), apply256times(fundef((a,), a.concat(s)))(Str("")))

solve = fundef(
    (rep100,),
    rep100(
        rep100(Str("D")).concat(rep100(Str("L"))).concat(rep100(Str("U"))).concat(rep100(Str("R")))
    ),
)

ans = Str("solve lambdaman8 ").concat(solve(rep100_fn))
print(ans)


# applytwice = fundef((f, x), f(f(x)))
# z = Var("z")
# apply256times = fundef((z,), z(z)(z))(applytwice)
# a = Var("a")
# rep100_fn = fundef((s,), apply256times(fundef((a,), a.concat(s)))(Str("")))
