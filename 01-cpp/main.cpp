#include <fstream>
#include <iostream>
#include <iterator>
#include <tuple>
#include <vector>

std::vector<double> getNumbers() {
  std::ifstream input("in.txt");
  std::istream_iterator<double> start(input), end;
  std::vector<double> numbers(start, end);
  return numbers;
}

std::tuple<int, int> solve(std::vector<double> n) {
  int partOne = 0, partTwo = 0;

  for (int i = 0; i < n.size(); i++) {
    if (i >= 1 && n[i] > n[i - 1])
      partOne++;
    if (i >= 3 && n[i] + n[i - 1] + n[i - 2] > n[i - 1] + n[i - 2] + n[i - 3])
      partTwo++;
  }

  return {partOne, partTwo};
}

int main() {
  int partOne = 0, partTwo = 0;
  std::vector<double> nums = getNumbers();
  std::tie(partOne, partTwo) = solve(nums);

  std::cout << "Day 1 part one: " << partOne << std::endl;
  std::cout << "Day 1 Part two: " << partTwo << std::endl;

  return 0;
}
