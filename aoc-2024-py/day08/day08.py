from collections import defaultdict
import numpy as np


def part1(grid):
    nodes = defaultdict(list)
    R = len(grid)
    C = len(grid[0])
    for r in range(R):
        for c in range(C):
            if (n := grid[r, c]) != ".":
                nodes[n].append((r, c))

    print(nodes)
    antinodes = set()
    for n, coords in nodes.items():
        for i in range(len(coords)):
            x1, y1 = coords[i]
            for j in range(i + 1, len(coords)):
                x2, y2 = coords[j]
                dx = x2 - x1
                dy = y2 - y1
                an1 = (x1 + 2 * dx, y1 + 2 * dy)
                an2 = (x2 - 2 * dx, y2 - 2 * dy)
                if 0 <= an1[0] < R and 0 <= an1[1] < C:
                    antinodes.add(an1)
                if 0 <= an2[0] < R and 0 <= an2[1] < C:
                    antinodes.add(an2)
    print(len(antinodes))


def part2(grid):
    nodes = defaultdict(list)
    R = len(grid)
    C = len(grid[0])
    for r in range(R):
        for c in range(C):
            if (n := grid[r, c]) != ".":
                nodes[n].append((r, c))

    print(nodes)
    antinodes = set()
    for n, coords in nodes.items():
        for i in range(len(coords)):
            x1, y1 = coords[i]
            for j in range(i + 1, len(coords)):
                x2, y2 = coords[j]
                dx = x2 - x1
                dy = y2 - y1
                for ix in range(C):
                    an1 = (x1 + ix * dx, y1 + ix * dy)
                    an2 = (x2 - ix * dx, y2 - ix * dy)
                    if 0 <= an1[0] < R and 0 <= an1[1] < C:
                        antinodes.add(an1)
                    if 0 <= an2[0] < R and 0 <= an2[1] < C:
                        antinodes.add(an2)
    print(len(antinodes))


def main() -> None:
    with open("./in.txt") as f:
        grid = np.array([list(line.strip()) for line in f.readlines()])

    part1(grid)
    part2(grid)


if __name__ == "__main__":
    main()
