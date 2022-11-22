import fs from "fs";

const getInput = () => {
  const [algo, img] = fs.readFileSync(`in.txt`, "utf8").trim().split("\n\n");

  return {
    algo: algo.split("").map((c) => c == "#"),
    img: img.split("\n").map((row) => row.split("").map((c) => c == "#")),
  };
};

// const formatImage = (img: boolean[][]) =>
//   img.map((row) => row.map((c) => (c ? "#" : ".")).join("")).join("\n");

const padImage = (img: boolean[][], lit: boolean) => {
  const padded = img.map((row) => [lit, ...row, lit]);
  padded.unshift(new Array(padded[0].length).fill(lit));
  padded.push(new Array(padded[0].length).fill(lit));
  return padded;
};

const getLitCount = (img: boolean[][]): number =>
  img.reduce((acc, row) => acc + row.filter((c) => c).length, 0);

const getSpotBinary = (img: boolean[][], x: number, y: number): number => {
  // for every spot, bit shift to the left and set the bit if the spot is lit.
  // this is way faster than converting to a string and parsing it as a binary number.
  return (
    [-1, 0, 1].reduce((acc, dy) => {
      [-1, 0, 1].forEach((dx) => {
        if (img[y + dy][x + dx]) acc++;
        acc = acc << 1;
      });
      return acc;
    }, 0) >> 1 // undo the last bit shift
  );
};

const iteration = (
  algo: boolean[],
  img: boolean[][],
  round: number
): number => {
  const next = padImage(img, round % 2 == 0);
  img = padImage(img, round % 2 == 1);
  for (let y = 1; y < img.length - 1; y++) {
    for (let x = 1; x < img[0].length - 1; x++) {
      const spot = getSpotBinary(img, x, y);
      next[y][x] = algo[spot];
    }
  }
  if (round == 1) return getLitCount(next);
  return iteration(algo, next, round - 1);
};

const solve = (algo: boolean[], img: boolean[][], rounds: number): number =>
  iteration(algo, padImage(img, false), rounds);

const { algo, img } = getInput();
console.log(`Day 20 part 1: ${solve(algo, img, 2)}`);
console.log(`Day 20 part 2: ${solve(algo, img, 50)}`);
