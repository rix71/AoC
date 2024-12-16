def parse(input):
    grid, moves = input.split("\n\n")
    moves = list("".join(moves.split("\n")))
    return grid, moves


def test1():
    with open("./test1.txt") as f:
        grid, moves = parse(f.read())
    res = part1(grid, moves)
    expected = 2028
    if res == expected:
        print(f"Test1 part1 passed! Answer: {res}")
    else:
        print(f"Test1 part1 failed! Expected: {expected}, Got: {res} ")


def test2():
    with open("./test2.txt") as f:
        grid, moves = parse(f.read())
    res = part1(grid, moves.copy())
    expected = 10092
    if res == expected:
        print(f"Test2 part1 passed! Answer: {res}")
    else:
        print(f"Test2 part1 failed! Expected: {expected}, Got: {res} ")
    res = part2(grid, moves.copy())
    expected = 9021
    if res == expected:
        print(f"Test2 part2 passed! Answer: {res}")
    else:
        print(f"Test2 part2 failed! Expected: {expected}, Got: {res} ")


DIRECTIONS = [(0, 1), (0, -1), (1, 0), (-1, 0)]
DIRECTIONS_SIGNS = [">", "<", "v", "^"]


def part1(grid, moves):
    grid = [list(row) for row in grid.split("\n")]
    rows = len(grid)
    cols = len(grid[0])

    walls = set()
    goods = set()

    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == "#":
                walls.add((r, c))
            elif grid[r][c] == "O":
                goods.add((r, c))
            elif grid[r][c] == "@":
                pos = (r, c)

    def move_goods(pos, direction):
        r, c = pos
        dr, dc = direction
        nr, nc = r + dr, c + dc
        if (nr, nc) in walls:
            return False
        if (nr, nc) in goods and not move_goods((nr, nc), direction):
            return False
        goods.remove((r, c))
        goods.add((nr, nc))
        return True

    while moves:
        r, c = pos
        move = moves.pop(0)
        (dr, dc) = DIRECTIONS[DIRECTIONS_SIGNS.index(move)]
        nr, nc = r + dr, c + dc
        if (nr, nc) in walls:
            continue
        if (nr, nc) in goods and not move_goods((nr, nc), (dr, dc)):
            continue
        pos = (nr, nc)

    result = 0
    for gr, gc in goods:
        result += 100 * gr + gc

    return result


def part2(grid, moves):
    grid = [row for row in grid.split("\n")]
    for i in range(len(grid)):
        grid[i] = (
            grid[i]
            .replace("#", "##")
            .replace("O", "[]")
            .replace(".", "..")
            .replace("@", "@.")
        )
    grid = [list(row) for row in grid]
    rows = len(grid)
    cols = len(grid[0])

    walls = set()
    goods = set()

    for r in range(rows):
        for c in range(0, cols, 2):
            if grid[r][c] == "#":
                walls.add((r, c))
            if grid[r][c + 1] == "#":
                walls.add((r, c + 1))
            elif "".join(grid[r][c : c + 2]) == "[]":
                goods.add(((r, c), (r, c + 1)))
            elif grid[r][c] == "@":
                pos = (r, c)

    def move_goods(pos, direction):
        goods.remove(pos)
        (lr, lc), (rr, rc) = pos
        dr, dc = direction
        nlr, nlc = lr + dr, lc + dc
        nrr, nrc = rr + dr, rc + dc
        if (nlr, nlc) in walls or (nrr, nrc) in walls:
            goods.add(pos)
            return False
        if ((nlr, nlc), (nrr, nrc)) in goods and not move_goods(
            ((nlr, nlc), (nrr, nrc)), direction
        ):
            goods.add(pos)
            return False
        if ((nlr, nlc + 1), (nrr, nrc + 1)) in goods and not move_goods(
            ((nlr, nlc + 1), (nrr, nrc + 1)), direction
        ):
            goods.add(pos)
            return False
        if ((nlr, nlc - 1), (nrr, nrc - 1)) in goods and not move_goods(
            ((nlr, nlc - 1), (nrr, nrc - 1)), direction
        ):
            goods.add(pos)
            return False
        goods.add(((nlr, nlc), (nrr, nrc)))
        return True

    while moves:
        r, c = pos
        move = moves.pop(0)
        (dr, dc) = DIRECTIONS[DIRECTIONS_SIGNS.index(move)]
        nr, nc = r + dr, c + dc
        if (nr, nc) in walls:
            continue

        if ((nr, nc), (nr, nc + 1)) in goods:
            goods_backup = goods.copy()
            if not move_goods(((nr, nc), (nr, nc + 1)), (dr, dc)):
                goods = goods_backup
                continue

        if ((nr, nc - 1), (nr, nc)) in goods:
            goods_backup = goods.copy()
            if not move_goods(((nr, nc - 1), (nr, nc)), (dr, dc)):
                goods = goods_backup
                continue
        pos = (nr, nc)

    result = 0
    for (lr, lc), (_, _) in goods:
        result += 100 * lr + lc

    return result


def main() -> None:
    test1()
    test2()

    with open("./in.txt") as f:
        grid, moves = parse(f.read())
    res = part1(grid, moves.copy())
    print(f"Part 1: {res}")
    res = part2(grid, moves.copy())
    print(f"Part 2: {res}")


if __name__ == "__main__":
    main()
