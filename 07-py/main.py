def getNumbers() -> list[int]:
    with open("in.txt", "r") as f:
        nums = list(map(int, f.readline().split(',')))
        nums.sort()
        return nums


def sum_dist(nums, x) -> int:
    return sum([(abs(n - x) * (abs(n - x) + 1)) // 2 for n in nums])


def solve(nums: list[int]) -> tuple[int, int]:
    median = nums[len(nums) // 2]
    partOne = sum(abs(x - median) for x in nums)

    mean = sum(nums) // len(nums)
    partTwo = min(sum_dist(nums, mean), sum_dist(nums, mean + 1))

    return (partOne, partTwo)


def main():
    partOne, partTwo = solve(getNumbers())
    print(f"Day 7 part one: {partOne}")
    print(f"Day 7 part two: {partTwo}")


if __name__ == "__main__":
    main()
