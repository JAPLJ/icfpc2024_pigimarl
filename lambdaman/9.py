import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *

s = Var("s")
cat = Var("C")
cat_fn = fundef((s,), s.concat(s).concat(s))

f = Var("f")
x = Var("x")
rep = Var("R")
rep_fn = fundef((f, x), f(f(f(x))))

rep27 = Var("Q")
rep27_fn = fundef((rep,), rep(cat_fn))(rep_fn)

solve = fundef(
    (rep27,), rep27(rep27(Str("RR")).concat(Str("D")).concat(rep27(Str("LL"))).concat(Str("D")))
)

ans = Str("solve lambdaman9 ").concat(solve(rep27_fn))
print(ans)
