def test1():
    with open("./test1.txt") as f:
        maze = [list(line.strip()) for line in f.readlines()]
    print(maze)
    res = part1(maze.copy())
    expected = 7036
    if res == expected:
        print(f"Test1 part1 passed! Answer: {res}")
    else:
        print(f"Test1 part1 failed! Expected: {expected}, Got: {res} ")


def test2():
    with open("./test2.txt") as f:
        maze = [list(line.strip()) for line in f.readlines()]
    print(maze)
    res = part1(maze.copy())
    expected = 11048
    if res == expected:
        print(f"Test2 part1 passed! Answer: {res}")
    else:
        print(f"Test2 part1 failed! Expected: {expected}, Got: {res} ")


DIRECTIONS = {
    0: (0, 1),  # EAST
    1: (1, 0),  # SOUTH
    2: (0, -1),  # WEST
    3: (-1, 0),  # NORTH
}


def part1(maze):
    rows = len(maze)
    cols = len(maze[0])

    for r in range(rows):
        for c in range(cols):
            if maze[r][c] == "S":
                print("Found start")
                start = (r, c)

    print(f"{start=}")
    paths = []
    tiles = []
    tiles.append((start, 0, 0))
    seen = set()
    while tiles:
        (r, c), cdir, pts = tiles.pop()
        # print(f"{r=}, {c=}, {cdir=}, {pts=}")
        seen.add(((r, c), cdir))
        for idir, (dr, dc) in DIRECTIONS.items():
            if (dr, dc) == DIRECTIONS[(cdir + 2) % 4]:
                continue  # ! Don't go back
            nr, nc = r + dr, c + dc
            if nr < 0 or nc < 0 or nr >= rows or nc >= cols:
                continue
            if maze[nr][nc] == "#" or ((nr, nc), idir) in seen:
                continue
            if maze[nr][nc] == "E":
                print(f"Found exit at {nr, nc} {pts + 1}")
                paths.append(pts + 1)
                continue
            if idir == cdir:
                tiles.append(((nr, nc), idir, pts + 1))
            else:
                tiles.append(((r, c), idir, pts + 1000))
    return min(paths)


def main() -> None:
    test1()
    test2()
    # with open("./test1.txt") as f:
    #     maze = [list(line.split()) for line in f.readlines()]
    # print(maze)


if __name__ == "__main__":
    main()
