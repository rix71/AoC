from itertools import product
import numpy as np
from scipy.optimize import minimize


def parse(input):
    machine_lines = [line.split("\n") for line in input.split("\n\n")]
    machines = []
    for line in machine_lines:
        ax, ay = (int(d.split("+")[1]) for d in line[0].split(": ")[1].split(", "))
        bx, by = (int(d.split("+")[1]) for d in line[1].split(": ")[1].split(", "))
        px, py = (int(d.split("=")[1]) for d in line[2].split(": ")[1].split(", "))
        machines.append(((ax, ay), (bx, by), (px, py)))
    return machines


def part1(machines):
    total_cost = 0
    for machine in machines:
        (ax, ay), (bx, by), (px, py) = machine

        steps = np.arange(0, 100, 1, dtype=int)
        btn_a_press = np.array([steps, ax * steps, ay * steps]).T
        btn_b_press = np.array([steps, bx * steps, by * steps]).T

        possible_combinations = []

        for a, b in product(btn_a_press, btn_b_press):
            if a[1] + b[1] == px and a[2] + b[2] == py:
                possible_combinations.append((a[0], b[0]))

        if len(possible_combinations) == 0:
            continue
        else:
            a, b = possible_combinations[0]
            cost = 3 * a + b
            for a, b in possible_combinations:
                if 3 * a + b < cost:
                    cost = 3 * a + b
            total_cost += cost
    print(f"Total Cost: {total_cost}")


def part2(machines):
    total_cost = 0
    for machine in machines:
        (ax, ay), (bx, by), (px, py) = machine
        px += 10000000000000
        py += 10000000000000

        a, b = np.linalg.solve(np.array([[ax, bx], [ay, by]]), np.array([px, py]))
        a = np.round(a)
        b = np.round(b)

        if a * ax + b * bx == px and a * ay + b * by == py:
            total_cost += 3 * a + b

    print(f"Total Cost: {int(total_cost)}")


def main() -> None:
    with open("./in.txt") as f:
        machines = parse(f.read())

    # print(machines)
    part1(machines)
    part2(machines)


if __name__ == "__main__":
    main()
