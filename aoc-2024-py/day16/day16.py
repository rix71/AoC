from collections import defaultdict
from heapq import heappush, heappop


def test1():
    with open("./test1.txt") as f:
        maze = [list(line.strip()) for line in f.readlines()]
    res = part1(maze.copy())
    expected = 7036
    if res == expected:
        print(f"Test1 part1 passed! Answer: {res}")
    else:
        print(f"Test1 part1 failed! Expected: {expected}, Got: {res} ")

    res = part2(maze.copy())
    expected = 45
    if res == expected:
        print(f"Test1 part2 passed! Answer: {res}")
    else:
        print(f"Test1 part2 failed! Expected: {expected}, Got: {res} ")


def test2():
    with open("./test2.txt") as f:
        maze = [list(line.strip()) for line in f.readlines()]
    res = part1(maze.copy())
    expected = 11048
    if res == expected:
        print(f"Test2 part1 passed! Answer: {res}")
    else:
        print(f"Test2 part1 failed! Expected: {expected}, Got: {res} ")

    res = part2(maze.copy())
    expected = 64
    if res == expected:
        print(f"Test2 part2 passed! Answer: {res}")
    else:
        print(f"Test2 part2 failed! Expected: {expected}, Got: {res} ")


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
                start = (r, c)
                break

    seen = set()
    tiles = []
    heappush(tiles, (0, start, 0))

    while tiles:
        pts, (r, c), cdir = heappop(tiles)
        seen.add(((r, c), cdir))
        for idir, (dr, dc) in DIRECTIONS.items():
            if (dr, dc) == DIRECTIONS[(cdir + 2) % 4]:
                continue
            nr, nc = r + dr, c + dc
            if nr < 0 or nc < 0 or nr >= rows or nc >= cols:
                continue
            if maze[nr][nc] == "#" or ((nr, nc), idir) in seen:
                continue
            if maze[nr][nc] == "E":
                return pts + 1
            if idir == cdir:
                heappush(tiles, (pts + 1, (nr, nc), idir))
            else:
                heappush(tiles, (pts + 1000, (r, c), idir))
    return 0


def part2(maze):
    rows = len(maze)
    cols = len(maze[0])

    for r in range(rows):
        for c in range(cols):
            if maze[r][c] == "S":
                start = (r, c)
                break

    on_path = defaultdict(set)
    seen = set()
    tiles = []
    heappush(tiles, (0, start, 0, {start}))
    
    while tiles:
        pts, (r, c), cdir, history = heappop(tiles)
        seen.add(((r, c), cdir))
        for idir, (dr, dc) in DIRECTIONS.items():
            if (dr, dc) == DIRECTIONS[(cdir + 2) % 4]:
                continue
            nr, nc = r + dr, c + dc
            if nr < 0 or nc < 0 or nr >= rows or nc >= cols:
                continue
            if maze[nr][nc] == "#" or ((nr, nc), idir) in seen:
                continue
            if maze[nr][nc] == "E":
                path_history = history.copy()
                path_history.add((nr, nc))
                on_path[pts + 1] |= path_history
                continue
            if idir == cdir:
                path_history = history.copy()
                path_history.add((nr, nc))
                heappush(tiles, (pts + 1, (nr, nc), idir, path_history))
            else:
                path_history = history.copy()
                path_history.add((r, c))
                heappush(tiles, (pts + 1000, (r, c), idir, path_history))
    answer = len(on_path[min(on_path.keys())])
    return answer


def main() -> None:
    test1()
    test2()

    with open("./in.txt") as f:
        maze = [list(line.strip()) for line in f.readlines()]

    res = part1(maze.copy())
    print(f"Part1: {res}")

    res = part2(maze.copy())
    print(f"Part2: {res}")


if __name__ == "__main__":
    main()
