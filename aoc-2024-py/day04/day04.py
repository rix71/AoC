import numpy as np


def part1(grid: np.ndarray):
    C, R = grid.shape
    count = 0
    for i in range(R):
        for j in range(C):
            if grid[i, j] == "X" or grid[i, j] == "S":
                wh = "".join(grid[i, j : j + 4])
                wv = "".join(grid[i : i + 4, j])
                count += 1 if (wh == "XMAS") or (wh == "SAMX") else 0
                count += 1 if (wv == "XMAS") or (wv == "SAMX") else 0
            if i + 4 <= R and j + 4 <= C:
                subgrid = grid[i : i + 4, j : j + 4]
                d1w = "".join(np.diag(subgrid))
                if d1w == "XMAS" or d1w == "SAMX":
                    count += 1
                d2w = "".join(np.diag(np.fliplr(subgrid)))
                if d2w == "XMAS" or d2w == "SAMX":
                    count += 1
    print(count)


def part2(grid: np.ndarray):
    C, R = grid.shape
    count = 0
    for i in range(R):
        for j in range(C):
            if i + 3 <= R and j + 3 <= C:
                subgrid = grid[i : i + 3, j : j + 3]
                d1w = "".join(np.diag(subgrid))
                d1ok = d1w == "MAS" or d1w == "SAM"
                d2w = "".join(np.diag(np.fliplr(subgrid)))
                d2ok = d2w == "MAS" or d2w == "SAM"
                if d1ok and d2ok:
                    count += 1
    print(count)


def main() -> None:
    with open("./in.txt") as file:
        lines = np.array([list(line.strip()) for line in file.readlines()])

    part1(lines)
    part2(lines)


if __name__ == "__main__":
    main()
