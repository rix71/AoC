import numpy as np


def parse(input):
    robot_lines = [line for line in input.split("\n")]
    robots = []
    for line in robot_lines:
        p, v = (
            (int(pv.split("=")[1].split(",")[0]), int(pv.split("=")[1].split(",")[1]))
            for pv in line.split(" ")
        )
        robots.append((p, v))
    return robots


def part1(robots):
    width = 11
    heigth = 7
    steps = 100
    quad = [0, 0, 0, 0]
    for robot in robots:
        (px, py), (vx, vy) = robot
        px = (px + vx * steps) % width
        py = (py + vy * steps) % heigth
        if px != (width // 2) and py != (heigth // 2):
            idx = 1 * (px > (width // 2)) + 2 * (py > (heigth // 2))
            quad[idx] += 1
    safety_factor = np.prod(quad)
    print(f"Safety factor: {safety_factor}")


def part2(robots):
    width = 101
    heigth = 103
    for i in range(1, 10000):
        seen = set()
        for robot in robots:
            (px, py), (vx, vy) = robot
            nx = (px + (vx * i)) % width
            ny = (py + (vy * i)) % heigth
            seen.add((nx, ny))
        if len(seen) == len(robots):
            print(f"Seconds: {i}")
            break


def main() -> None:
    with open("./in.txt") as f:
        robots = parse(f.read())

    print(robots)
    part1(robots)
    part2(robots)


if __name__ == "__main__":
    main()
