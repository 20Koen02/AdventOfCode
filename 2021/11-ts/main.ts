import fs from "fs";

type Matrix = number[][];
interface Octopuses {
  matrix: Matrix;
  flashes: number;
}
const getMatrix = (): Matrix =>
  fs
    .readFileSync(`in.txt`, "utf8")
    .split("\n")
    .map((line) => line.split("").map(Number));

const cartesianProduct = (a: any[], b: any[]) =>
  a.reduce((p, x) => [...p, ...b.map((y) => [x, y])], []);

function evolve(op: Octopuses, y: number, x: number): Octopuses {
  if (y < 0 || y >= 10 || x < 0 || x >= 10) return op;
  op.matrix[y][x]++;
  if (op.matrix[y][x] == 10) {
    op.flashes++;
    cartesianProduct([-1, 0, 1], [-1, 0, 1]).forEach(
      ([dy, dx]: [number, number]) => (op = evolve(op, y + dy, x + dx))
    );
  }
  return op;
}

function solve(op: Octopuses): [number, number] {
  let partOne = 0,
    partTwo = 0;

  let i = 1;
  while (true) {
    op.matrix.forEach((row, y) =>
      row.forEach((_, x) => (op = evolve(op, y, x)))
    );

    let flashedAll = true;
    op.matrix = op.matrix.map((row) =>
      row.map((cell) => {
        if (cell > 9) {
          return 0;
        } else {
          flashedAll = false;
        }
        return cell;
      })
    );

    if (flashedAll) {
      partTwo = i;
      break;
    }
    if (i == 100) partOne = op.flashes;
    i++;
  }

  return [partOne, partTwo];
}

let [partOne, partTwo] = solve({ matrix: getMatrix(), flashes: 0 });
console.log(`Day 11 part one: ${partOne}`);
console.log(`Day 11 part two: ${partTwo}`);
