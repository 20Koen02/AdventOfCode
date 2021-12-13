import fs from "fs";

interface Dot {
  x: number;
  y: number;
}
interface Fold {
  axis: "x" | "y";
  value: number;
}
interface Instructions {
  dots: Dot[];
  folds: Fold[];
}

const getInstructions = (): Instructions => {
  const [dotsStr, foldsStr] = fs.readFileSync(`in.txt`, "utf8").split("\n\n");
  return {
    dots: dotsStr.split("\n").map((line) => {
      const [x, y] = line.split(",").map(Number);
      return { x, y } as Dot;
    }),
    folds: foldsStr.split("\n").map((line) => {
      const [axisStr, value] = line.split("=");
      return {
        axis: axisStr[axisStr.length - 1],
        value: Number(value),
      } as Fold;
    }),
  } as Instructions;
};

const noOverlap = (dot: Dot, dots: Dot[]): boolean =>
  !dots.filter(({ x, y }) => x == dot.x && y == dot.y).length;

const printDots = (dots: Dot[]): string => {
  let result = "";
  let [w, h] = dots.reduce(
    (acc, dot) => [Math.max(acc[0], dot.x), Math.max(acc[1], dot.y)],
    [0, 0]
  );
  for (let y = 0; y <= h; y++) {
    for (let x = 0; x <= w; x++) {
      result += noOverlap({ x, y }, dots) ? " " : "#";
    }
    result += "\n";
  }
  return result;
};

function fold(instr: Instructions): Instructions {
  const fold = instr.folds.shift()!;
  const newDots: Dot[] = [];
  instr.dots.forEach((dot) => {
    if (dot[fold.axis] > fold.value) {
      const newDot: Dot = {
        x: fold.axis === "x" ? fold.value * 2 - dot.x : dot.x,
        y: fold.axis === "y" ? fold.value * 2 - dot.y : dot.y,
      };
      if (noOverlap(newDot, instr.dots)) newDots.push(newDot);
    } else newDots.push(dot);
  });
  instr.dots = newDots;
  return instr;
}

const partOne = (instr: Instructions): number => fold(instr).dots.length;

const partTwo = (instr: Instructions): string => {
  while (instr.folds.length > 0) instr = fold(instr);
  return printDots(instr.dots);
};

const instructions = getInstructions();
console.log(`Day 13 part one: ${partOne(instructions)}`);
console.log(`Day 13 part two: \n${partTwo(instructions)}`);
