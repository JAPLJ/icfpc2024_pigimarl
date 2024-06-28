T = """abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ \n"""
c2i = {chr(i + 33): i for i in range(94)}


# 人間が読めるように
def conv(s):
    return "".join(T[ord(x) - 33] for x in s)


# lang2icfp
def inv(s):
    return "".join(chr(T.index(x) + 33) for x in s)


def to_i(s: str):
    n = 0
    for i, c in enumerate(reversed(s)):
        n += c2i[c] * 94**i
    return n


def to_I(i: int):
    s = ""
    while i >= 94:
        m = i % 94
        i = i // 94
        s = chr(m + 33) + s
    s = chr(i + 33) + s
    return "I" + s
