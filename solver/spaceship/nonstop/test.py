from pathlib import Path
import sys
sys.path.append(str(Path(__file__).parent))

from main import calc_move1d, join_xy_moves, move_on_time


def test_calc_move1d():
    test_cases = [
        ([0, 1, 0], [[1], 1]),
        ([0, 5, 0], [[1, 1, 0], 2]),
        ([0, 5, 1], [[1, 1], 3]),
        ([0, -5, -1], [[-1, -1], -3]),
        ([1, 5, 1], [[1, 0], 2]),
        ([0, 15, 0], [[1, 1, 1, 1, 1], 5]),
        ([0, 5, 5], [[0], 5]),
        ([0, 6, 5], [[1], 6]),
        ([0, 4, 5], [[-1], 4]),
        ([0, 100, 10], [[-1, 1, 1, 1, 1, 1, 1, 1], 16]),
        ([0, 1, 4], [[-1, -1, -1, -1, -1, -1, -1, 0], -3]),
        ([0, 35, 40], [[-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], -39]),
        ([0, 35, 20], [[-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], -19]),
        ([4, 7, 2], [[1], 3]),
        ([3, 4, 2], [[-1], 1]),
    ]

    for args, expected in test_cases:
        result = calc_move1d(*args)
        assert result == expected, f"assertion error at test case {args}: {result=}, {expected=}"


def test_join_xy_moves():
    test_cases = [
        ([[1, 1, 0], [0, 1, 0]], ["6", "9", "5"]),
        ([[1, 0, 1], [0, 1, 1]], ["6", "8", "9"]),
    ]

    for args, expected in test_cases:
        result = join_xy_moves(*args)
        assert result == expected, f"assertion error at test case {args}: {result=}, {expected=}"


def test_move_on_time():
    test_cases = [
        ([0, 3, 0, 3], [[0, 1, 1], 2]),
        ([0, 4, 0, 3], [[1, 0, 1], 2]),
        ([-20, -20, -4, 1], [None, None]),
    ]

    for args, expected in test_cases:
        result = move_on_time(*args)
        assert result == expected, f"assertion error at test case {args}: {result=}, {expected=}"


if __name__ == "__main__":
    test_calc_move1d()
    test_join_xy_moves()
    test_move_on_time()
    print("test passed")
