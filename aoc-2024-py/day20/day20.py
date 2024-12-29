from collections import defaultdict
from heapq import heappop, heappush


DIRECTIONS = [
    (0, 1),  # EAST
    (1, 0),  # SOUTH
    (0, -1),  # WEST
    (-1, 0),  # NORTH
]


def reconstruct_path(prev, start, end):
    track = []
    while end != start:
        track.append(end)
        assert len(prev[end]) == 1
        end = prev[end][0]
    track.append(start)
    return track[::-1]


def find_path(maze, start, end):
    rows = len(maze)
    cols = len(maze[0])

    prev = defaultdict(list)
    q = [(0, start)]
    visited = set()
    while q:
        steps, (r, c) = heappop(q)
        for dr, dc in DIRECTIONS:
            nr, nc = r + dr, c + dc
            if nr < 0 or nc < 0 or nr >= rows or nc >= cols:
                continue
            if maze[nr][nc] == "#" or (nr, nc) in visited:
                continue
            if (nr, nc) == end:
                prev[(nr, nc)].append((r, c))
                track = reconstruct_path(prev, start, end)
                return track, steps + 1
            heappush(q, (steps + 1, (nr, nc)))
            visited.add((nr, nc))
            prev[(nr, nc)].append((r, c))
    assert False, "No path found"


def part1(maze):
    rows = len(maze)
    cols = len(maze[0])

    for r in range(rows):
        for c in range(cols):
            if maze[r][c] == "S":
                start = (r, c)
            elif maze[r][c] == "E":
                end = (r, c)

    normal_track, _ = find_path(maze, start, end)

    cheats = defaultdict(int)
    for start_cost, (r, c) in enumerate(normal_track):
        for dr, dc in DIRECTIONS:
            nr, nc = r + dr, c + dc
            nnr, nnc = nr + dr, nc + dc
            if maze[nr][nc] != "#" or (nnr, nnc) not in normal_track:
                continue
            if (cheat_end_idx := normal_track.index((nnr, nnc))) > start_cost:
                cheat_win = cheat_end_idx - (start_cost + 1)
                cheats[cheat_win] += 1

    answer = 0
    for win, count in cheats.items():
        if win >= 100:
            answer += count
    print(answer)


def part2(maze):
    rows = len(maze)
    cols = len(maze[0])

    for r in range(rows):
        for c in range(cols):
            if maze[r][c] == "S":
                start = (r, c)
            elif maze[r][c] == "E":
                end = (r, c)

    normal_track, normal_steps = find_path(maze, start, end)

    cheats = defaultdict(int)
    for start_cost, (r, c) in enumerate(normal_track[:-1]):
        for end_cost, (nr, nc) in enumerate(
            normal_track[start_cost + 1 :], start=start_cost + 1
        ):
            distance = abs(r - nr) + abs(c - nc)
            if distance > 20:
                continue
            cheat_cost = start_cost + (normal_steps - end_cost) + distance
            cheats[normal_steps - cheat_cost] += 1

    answer = 0
    for win, count in cheats.items():
        if win >= 100:
            answer += count
    print(answer)


def main() -> None:
    maze = [list(line.strip()) for line in open("./in.txt").readlines()]
    part1(maze)
    part2(maze)


if __name__ == "__main__":
    main()
