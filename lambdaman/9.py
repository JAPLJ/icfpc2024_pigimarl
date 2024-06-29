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

d = Var("d")
row = Var("O")
row_fn = fundef((cat, rep, d), rep(cat)(d).concat(Str("D")))

fill = fundef((row, cat, rep), rep(cat)(row(cat)(rep)(Str("RR")).concat(row(cat)(rep)(Str("LL")))))
res = fill(row_fn)(cat_fn)(rep_fn)

ans = Str("solve lambdaman9 ").concat(res)
print(ans)
