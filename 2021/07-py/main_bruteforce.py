def getNumbers() -> list[int]:
    with open("in.txt", "r") as f:
        return list(map(int, f.readline().strip().split(",")))


def simulate(nums: list[int], alignment: int, linear: bool = True) -> int:
    fuel = 0
    for num in nums:
        distance = abs(num - alignment)
        fuel += distance if linear else (distance ** 2 + distance) / 2
    return int(fuel)


def solve(nums: list[int], linear: bool = True) -> int:
    result = None
    for i in range(min(nums), max(nums)):
        sim = simulate(nums, i, linear)
        result = min(result, sim) if result else sim
    return result


def main():
    print(f"Day 7 part one: {solve(getNumbers())}")
    print(f"Day 7 part two: {solve(getNumbers(), False)}")


if __name__ == "__main__":
    main()
