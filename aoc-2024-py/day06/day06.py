def part1(grid):
    R = len(grid)
    C = len(grid[0])

    for i in range(R):
        for j in range(C):
            if grid[i][j] in "^>V<":
                ci, cj = i, j
                cd = "^>V<".find(grid[i][j])

    print(ci, cj, cd)
    dirs = [
        (-1, 0),  # UP
        (0, 1),  # RIGHT
        (1, 0),  # DOWN
        (0, -1),  # LEFT
    ]

    seen = set()
    while True:
        seen.add((ci, cj))
        di, dj = dirs[cd]
        ci += di
        cj += dj
        if ci < 0 or ci >= R or cj < 0 or cj >= C:
            break
        if grid[ci][cj] == "#":
            cd = (cd + 1) % 4
            ci -= di
            cj -= dj

    count = len(seen)
    print(count)


def part2(grid):
    R = len(grid)
    C = len(grid[0])

    for i in range(R):
        for j in range(C):
            if grid[i][j] in "^>V<":
                ci, cj = i, j
                cd = "^>V<".find(grid[i][j])

    print(ci, cj, cd)
    dirs = [
        (-1, 0),  # UP
        (0, 1),  # RIGHT
        (1, 0),  # DOWN
        (0, -1),  # LEFT
    ]

    def loops(grid, ci, cj, cd):
        di, dj = dirs[cd]
        seen = set()
        while True:
            seen.add((ci, cj, di, dj))
            if ci + di < 0 or ci + di >= R or cj + dj < 0 or cj + dj >= C:
                return False
            if grid[ci+di][cj+dj] == "#":
                dj, di = -di, dj
            else:
                ci += di
                cj += dj
            if (ci, cj, di, dj) in seen:
                return True

    count = 0
    for r in range(R):
        for c in range(C):
            if grid[r][c] != ".":
                continue
            grid[r][c] = "#"
            if loops(grid, ci, cj, cd):
                count += 1
            grid[r][c] = "."

    print(count)


def main() -> None:
    with open("./in.txt") as file:
        lines = [list(line.strip()) for line in file.readlines()]

    # print(lines)
    part1(lines)
    part2(lines)


if __name__ == "__main__":
    main()
