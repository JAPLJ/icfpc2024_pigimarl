from common.icfp import to_I, inv


class Expr:
    def __neg__(self):
        return UnaryOp("-", self)

    def lnot(self):
        return UnaryOp("!", self)

    def to_int(self):
        return UnaryOp("#", self)

    def to_str(self):
        return UnaryOp("$", self)

    def __add__(self, other):
        return BinaryOp("+", self, other)

    def __sub__(self, other):
        return BinaryOp("-", self, other)

    def __mul__(self, other):
        return BinaryOp("*", self, other)

    def __truediv__(self, other):
        return BinaryOp("/", self, other)

    def __mod__(self, other):
        return BinaryOp("%", self, other)

    def __lt__(self, other):
        return BinaryOp("<", self, other)

    def __gt__(self, other):
        return BinaryOp(">", self, other)

    def __eq__(self, other):
        return BinaryOp("=", self, other)

    def __or__(self, other):
        return BinaryOp("|", self, other)

    def __and__(self, other):
        return BinaryOp("&", self, other)

    def concat(self, other):
        return BinaryOp(".", self, other)

    def take(self, other):
        return BinaryOp("T", other, self)

    def drop(self, other):
        return BinaryOp("D", other, self)

    def __call__(self, other):
        return BinaryOp("$", self, other)


class Bool(Expr):
    def __init__(self, value):
        self.value = value

    def __str__(self):
        return "T" if self.value else "F"


class Int(Expr):
    def __init__(self, value):
        self.value = value

    def __str__(self):
        return to_I(self.value)


class Str(Expr):
    def __init__(self, value):
        self.value = value

    def __str__(self):
        return "S" + inv(self.value)


class UnaryOp(Expr):
    def __init__(self, op, expr):
        self.op = op
        self.expr = expr

    def __str__(self):
        return f"U{self.op} {self.expr}"


class BinaryOp(Expr):
    def __init__(self, op, left, right):
        self.op = op
        self.left = left
        self.right = right

    def __str__(self):
        return f"B{self.op} {self.left} {self.right}"


class If(Expr):
    def __init__(self, cond, left, right):
        self.cond = cond
        self.left = left
        self.right = right

    def __str__(self):
        return f"? {self.cond} {self.left} {self.right}"


class Lambda(Expr):
    def __init__(self, v, expr):
        self.v = v
        self.expr = expr

    def __str__(self):
        return f"L{self.v.v} {self.expr}"


class Var(Expr):
    def __init__(self, v):
        self.v = v

    def __str__(self):
        return f"v{self.v}"


def fundef(args, body):
    if not args:
        return body
    return Lambda(args[0], fundef(args[1:], body))
