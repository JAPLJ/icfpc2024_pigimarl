import os, sys

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.trans import *


s = Var("s")
cat = fundef((s,), s.concat(s).concat(s))

f = Var("f")
trip = fundef((f,), f(f(f(Str("RRRRRRRR")))))

ans = Str("solve lambdaman6 ").concat(trip(cat))
print(ans)
