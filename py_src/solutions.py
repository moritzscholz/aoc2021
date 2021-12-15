from pathlib import Path
from polymerization import Polymerizer


def main():
    # Day 14
    polymerizer = Polymerizer(Path("data/day14/input.txt"))
    for _ in range(0, 10):
        polymerizer.apply_rules()

    print(f"Answer for day 14: max - min frequency = {polymerizer.answer()}")


if __name__ == "__main__":
    main()
