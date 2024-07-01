import sympy


def fib(n):
    a, b = 1, 1
    for i in range(n):
        a, b = b, a + b
    return a


for g in range(31, 100):
    f = fib(g)
    if sympy.isprime(f):
        print(g, f)
        break
