#include <fstream>
#include <string>

int partOne() {
  int prev = 0, timesIncreases = 0;

  std::ifstream input("in.txt");
  for (std::string line; getline(input, line);) {
    int cur = std::stoi(line);
    if (prev && cur > prev) {
      timesIncreases++;
    }
    prev = cur;
  }

  return timesIncreases;
}

int partTwo() {
  int prevSlidingWindow = 0, timesIncreases = 0, prev = 0, prevprev = 0;

  std::ifstream input("in.txt");
  for (std::string line; getline(input, line);) {
    int cur = std::stoi(line);

    if (!prevprev) {
      prevprev = cur;
      continue;
    }
    if (!prev) {
      prev = cur;
      continue;
    }

    int slidingWindow = cur + prev + prevprev;

    if (prevSlidingWindow && slidingWindow > prevSlidingWindow) {
      timesIncreases++;
    }
    prevSlidingWindow = slidingWindow;
    prevprev = prev;
    prev = cur;
  }

  return timesIncreases;
}

int main() {
  printf("Day 1 part one: %d\n", partOne());
  printf("Day 1 Part two: %d\n", partTwo());

  return 0;
}
