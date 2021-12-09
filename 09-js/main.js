const fs = require("fs");

const readParts = () =>
  fs
    .readFileSync(`in.txt`, "utf8")
    .split("\n")
    .map((line) => line.split("").map(Number));

function getBasinSize(matrix, coords) {
  let basin = [];
  const search = (coords) => {
    if (!basin.filter(({ x, y }) => x == coords.x && y == coords.y).length)
      basin.push(coords);
    [...adjacentGenerator(matrix, coords)]
      .filter(
        ({ x, y }) =>
          matrix[y][x] < 9 && matrix[y][x] > matrix[coords.y][coords.x]
      )
      .forEach(search);
  };
  search(coords);
  return basin.length;
}

function* adjacentGenerator(matrix, { x, y }) {
  if (x > 0) yield { x: x - 1, y };
  if (x < matrix[0].length - 1) yield { x: x + 1, y };
  if (y > 0) yield { x, y: y - 1 };
  if (y < matrix.length - 1) yield { x, y: y + 1 };
}

function* lowNumsGenerator(matrix) {
  for (let y = 0; y < matrix.length; y++) {
    for (let x = 0; x < matrix[y].length; x++) {
      const isLowest = [...adjacentGenerator(matrix, { x, y })].every(
        (el) => matrix[y][x] < matrix[el.y][el.x]
      );
      if (isLowest) yield { x, y };
    }
  }
}

const getLowNums = (matrix) => [...lowNumsGenerator(matrix)];

const partOne = (matrix, lowNums) =>
  lowNums.map(({ x, y }) => matrix[y][x] + 1).reduce((p, c) => p + c);

const partTwo = (matrix, lowNums) =>
  lowNums
    .map((coords) => getBasinSize(matrix, coords))
    .sort((a, b) => a - b)
    .slice(-3)
    .reduce((p, c) => p * c);

const matrix = readParts();
const lowNums = getLowNums(matrix);
console.log(`Day 9 part one: ${partOne(matrix, lowNums)}`);
console.log(`Day 9 part two: ${partTwo(matrix, lowNums)}`);
