from collections import defaultdict
from itertools import product


def part1(net_map) -> int:
    conns = defaultdict(set)
    for a, b in net_map:
        conns[a].add(b)
        conns[b].add(a)

    nets = set()
    for a, b, c in product(conns.keys(), conns.keys(), conns.keys()):
        if (a == b) or (b == c) or (a == c):
            continue
        if b in conns[a] and c in conns[b] and a in conns[c]:
            nets.add(tuple(sorted([a, b, c])))

    ans = 0
    for n in nets:
        if any(c.startswith("t") for c in n):
            ans += 1
    return ans


def part2(net_map) -> str:
    conns = defaultdict(set)
    for a, b in net_map:
        conns[a].add(b)
        conns[b].add(a)

    nets = set()
    for a in conns:
        anet = {c: conns[c] for c in conns[a]}
        keys = list(anet.keys())
        keys.append(a)
        for b in keys:
            for c in anet:
                if b == c:
                    continue
                if b not in anet[c]:
                    if c in keys:
                        keys.remove(c)
        nets.add(tuple(sorted(keys)))

    return ",".join(max(nets, key=len))


def main() -> None:
    with open("./in.txt") as f:
        net_map = [line.strip().split("-") for line in f.readlines()]

    res = part1(net_map)
    print(f"Part 1: {res}")

    res = part2(net_map)
    print(f"Part 2: {res}")


if __name__ == "__main__":
    main()
