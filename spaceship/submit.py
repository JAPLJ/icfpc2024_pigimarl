import os, sys
import requests

sys.path.append(os.path.join(os.path.dirname(__file__), ".."))

from common.icfp import *

case_n = int(sys.argv[1])
commands = open(sys.argv[2]).read().strip()

body = "S" + inv(f"solve spaceship{case_n} {commands}")

uri = "https://boundvariable.space/communicate"
headers = {"Authorization": "Bearer 29089507-8721-4390-a1c4-55d5db6398d8"}
res = requests.post(uri, headers=headers, data=body)

t = res.text
print(t)
if t.startswith("S"):
    print(conv(t[1:]))
