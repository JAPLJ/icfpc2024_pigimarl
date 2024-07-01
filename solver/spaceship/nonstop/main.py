import os
from itertools import product
from collections import defaultdict, deque


class PathFinder:
    vecs = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (0, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ]
    def __init__(self, points):
        self.points_by_x = defaultdict(set)
        for i, point in enumerate(points):
            x, y = point
            self.points_by_x[x].add(y)
        self.count = sum(len(ys) for ys in self.points_by_x.values())

    def get_next_point(self, x, y, vx, vy, t_max=3, area=3):
        # 09
        if (x, y) == (137, 165):
            x, y = 136, 159
        elif (x, y) == (135, 158):
            x, y = 134, 162
        elif (x, y) == (134, 162):
            x, y = 133, 156
        elif (x, y) == (81, 61):
            x, y = 84, 65
        for t in range(1, t_max + 1):
            xi = x + vx * t
            yi = y + vy * t
            q = deque()
            q.append((xi, yi))
            visited = set()
            while len(q) > 0:
                xi, yi = q.popleft()
                if abs(xi - x) > area or abs(yi - y) > area:
                    continue
                if (xi, yi) in visited:
                    continue
                if xi in self.points_by_x and yi in self.points_by_x[xi]:
                    self.count -= 1
                    self.points_by_x[xi].remove(yi)
                    return xi, yi
                visited.add((xi, yi))
                for vec in self.vecs:
                    q.append((xi + vec[0], yi + vec[1]))
            return None, None


def solve(pf):
    commands = []
    x, y = 0, 0
    vx, vy = 0, 0
    # pf.count -= 10
    while pf.count > 0:
        point = pf.get_next_point(x, y, vx, vy, t_max=10, area=40)
        if point == (None, None):
            print("no next point", x, y, vx, vy)
            return commands + ["5"] * 3
        x_move, vx_ = calc_move1d(x, point[0], vx)
        y_move, vy_ = calc_move1d(y, point[1], vy)
        length = max(len(x_move), len(y_move))
        # 06
        # if (x, y) == (-11, 207):
        #     length = 3
        # if (x, y) == (-92, 287):
        #     length = 3
        # if (x, y) == (-97, 301):
        #     length = 4
        # if (x, y) == (-121, 350):
        #     length += 1
        #  10
        # if (x, y) == (-189, -153):
        #     length += 1
        # if (x, y) == (-631, -388):
        #     length += 1
        # if (x, y) == (-1000, -579):
        #     length += 1
        # if (x, y) == (-1038, -606):
        #     length += 1
        # if (x, y) == (-1171, -544):
        #     length += 1
        # if (x, y) == (-1228, -536):
        #     length += 0
        # if (x, y) == (-1242, -531):
        #     length += 1
        # 09
        if (x, y) == (7, 12):
            length += 2
        if (x, y) == (41, 49):
            length += 2
        if (x, y) == (63, 57):
            length += 1
        if (x, y) == (148, 153):
            length += 1
        if (x, y) == (144, 172):
            length += 1
        if (x, y) == (-21, -101):
            length += 1
        for li in range(length, length + 100):
            vxi, vyi = vx, vy
            if False:
                pass
            # 06
            # if (x, y) == (-121, 350):
            #     x_movei, vxi = move_on_time(x, point[0], vx, li)[0]
            #     y_movei, vyi = move_on_time(y, point[1], vy, li, 4)[3]
            # 10
            # if (x, y) == (-1242, -531):
            #     x_movei, vxi = move_on_time(x, point[0], vx, li)[0]
            #     y_movei, vyi = move_on_time(y, point[1], vy, li, 4)[2]
            # 09
            elif (x, y) == (7, 12):
                x_movei, vxi = move_on_time(x, point[0], vx, li)[0]
                y_movei, vyi = move_on_time(y, point[1], vy, li, 10)[0]
            elif (x, y) == (148, 153):
                x_movei, vxi = move_on_time(x, point[0], vx, li)[0]
                y_movei, vyi = move_on_time(y, point[1], vy, li, 10)[1]
            elif (x, y) == (-21, -101):
                x_movei, vxi = move_on_time(x, point[0], vx, li)[0]
                y_movei, vyi = move_on_time(y, point[1], vy, li, 10)[0]
            else:
                x_movei, vxi = move_on_time(x, point[0], vx, li)[0]
                y_movei, vyi = move_on_time(y, point[1], vy, li)[0]
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
def move_on_time(s, t, v, length, n=1):
    res = []
    for p in product([-1, 0, 1], repeat=length):
        if len(res) >= n:
            break
        pos = s
        vi = v
        for dv in p:
            vi += dv
            pos += vi
        if pos == t:
            res.append([p, vi])
    if res:
        return res
    return [[None, None]]
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
    fname = os.path.join(os.path.dirname(__file__), "../../../problems/spaceship/09.txt")
    with open(fname, "r") as f:
        for line in f:
            if line.strip() == "":
                continue
            points.append(list(map(int, line.strip().split())))
    # sorted_points = sorted(points, key=lambda x: -x[0])
    # for p in sorted_points:
    #     print(p[0], p[1])

    pf = PathFinder(points)
    commands = solve(pf)
    print("".join(commands))
    print(len("".join(commands)))
