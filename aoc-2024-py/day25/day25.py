from itertools import product
import numpy as np

def part1(keys, locks) -> None:

    h, _ = keys[0].shape

    lock_heights = [np.sum(l, axis=0) - 1 for l in locks]
    key_heights = [np.sum(k, axis=0) - 1 for k in keys]

    matches = 0
    for l, k in product(lock_heights, key_heights):
        if np.all(l + k < h - 1):
            matches += 1

    print(matches)


def main() -> None:
    with open("./in.txt", "r") as file:
        keys = []
        locks = []
        for schem in file.read().split("\n\n"):
            s = np.array([list(x) for x in schem.split("\n")])
            s = np.where(s == "#", 1, 0)
            if np.all(s[0, :] == 0):
                keys.append(s)
            else:
                locks.append(s)

    part1(keys, locks)


if __name__ == "__main__":
    main()
