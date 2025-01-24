from collections import defaultdict
from functools import lru_cache


@lru_cache
def sequence(n: int) -> int:
    n = ((n << 6) ^ n) % 16777216
    n = ((n >> 5) ^ n) % 16777216
    n = ((n << 11) ^ n) % 16777216
    return n


def part1(initial):
    ans = 0
    for n in initial:
        _n = n
        for _ in range(2000):
            _n = sequence(_n)
        ans += _n
    return ans


def part2(initial):
    total_bananas = defaultdict(int)
    for n in initial:
        seq = []
        changes = []
        for _ in range(2000):
            seq.append(n)
            _n = sequence(n)
            changes.append((_n % 10) - (n % 10))
            n = _n
        seq.append(n)

        seen = set()
        for i in range(2000 - 3):
            if (chsub := tuple(changes[i : i + 4])) not in seen:
                total_bananas[chsub] += seq[i + 4] % 10
                seen.add(chsub)

    return max(total_bananas.values())


def main() -> None:

    with open("./test.txt") as f:
        initial = [int(line.strip()) for line in f.readlines()]

    res = part1(initial)
    print(f"Part 1: {res}")

    res = part2(initial)
    print(f"Part 2: {res}")


if __name__ == "__main__":
    main()
