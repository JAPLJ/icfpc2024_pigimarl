import os
from itertools import product


def solve(points):
    sorted_points = sorted(points, key=lambda x: x[1])

    commands = []
    x, y = 0, 0
    vx, vy = 0, 0
    for point in sorted_points:
        x_move, vx_ = calc_move1d(x, point[0], vx)
        y_move, vy_ = calc_move1d(y, point[1], vy)
        length = max(len(x_move), len(y_move))
        for li in range(length, length + 100):
            vxi, vyi = vx, vy
            x_movei, vxi = move_on_time(x, point[0], vx, li)
            y_movei, vyi = move_on_time(y, point[1], vy, li)
            if x_movei and y_movei:
                vx = vxi
                vy = vyi
                x_move = x_movei
                y_move = y_movei
                break
        if not x_move or not y_move:
            raise RuntimeError("no solution", x, y, point[0], point[1], vx, vy)
        moves = join_xy_moves(x_move, y_move)
        commands.append("".join(moves))
        x, y = point
    return commands


# s から t まで最速で移動する
# s: start, t: target, v: velocity
def calc_move1d(s, t, v):
    if s == t:
        return ([], v)
    diff = abs(t - s)
    if s > t:
        v = -v
    dp = [None for _ in range(diff + 1)]
    dp[0] = [[], v]
    for i in range(diff + 1):
        if dp[i] is None:
            continue
        mvs = dp[i][0]
        v = dp[i][1]
        for dv in range(-1, 2):
            j = i + v + dv
            if j < 0 or j > diff:
                continue
            if dp[j] and len(dp[j][0]) <= len(mvs) + 1:
                continue
            dp[j] = [mvs + [dv], v + dv]
    # 速すぎてうまく止まれない場合
    if dp[diff] is None:
        moves_to_stop = [-1] * v
        stop_position = s + v * (v + 1) // 2
        mvs, v = calc_move1d(stop_position, t, 0)
        return [moves_to_stop + mvs, v]
    if s > t:
        dp[diff][0] = [-x for x in dp[diff][0]]
        dp[diff][1] = -dp[diff][1]
    return dp[diff]


# s から t まで length 時間で移動する
def move_on_time(s, t, v, length):
    for p in product([-1, 0, 1], repeat=length):
        pos = s
        vi = v
        for dv in p:
            vi += dv
            pos += vi
        if pos == t:
            return [list(p), vi]
    return [None, None]
    # raise RuntimeError("no solution", s, t, v, length)


# x と y の移動を結合する
# x_move, y_move はそれぞれ x, y 方向の移動
def join_xy_moves(x_move, y_move):
    moves = []
    for x, y in zip(x_move, y_move):
        x = x or 0
        y = y or 0
        if x == 0 and y == 0:
            moves.append("5")
        elif x == 0 and y == 1:
            moves.append("8")
        elif x == 0 and y == -1:
            moves.append("2")
        elif x == 1 and y == 0:
            moves.append("6")
        elif x == 1 and y == 1:
            moves.append("9")
        elif x == 1 and y == -1:
            moves.append("3")
        elif x == -1 and y == 0:
            moves.append("4")
        elif x == -1 and y == 1:
            moves.append("7")
        elif x == -1 and y == -1:
            moves.append("1")
        else:
            raise RuntimeError("invalid move", x, y)
    return moves


if __name__ == "__main__":
    points = []
    fname = os.path.join(os.path.dirname(__file__), "../../../problems/spaceship/06.txt")
    with open(fname, "r") as f:
        for line in f:
            if line.strip() == "":
                continue
            points.append(list(map(int, line.strip().split())))
    commands = solve(points)
    print("".join(commands))
