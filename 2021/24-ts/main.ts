import fs from "fs";

// don't need the vm but it's nice to have
import { VM } from "./vm";

class IntTriplet {
  constructor(public idx: number, public x: number, public y: number) {}
}

const getInstructions = (): string[] => {
  const instructions: string[] = fs.readFileSync(`in.txt`, "utf8").split("\n");
  return instructions;
};

const getKV = (instructions: string[]) => {
  let keyValues: IntTriplet[] = [];

  for (
    let idx = 0, i = 5, j = 15;
    i < instructions.length && j < instructions.length;
    idx++, i += 18, j += 18
  ) {
    let x = +instructions[i].split(" ")[2];
    let y = +instructions[j].split(" ")[2];

    keyValues.push(new IntTriplet(idx, x, y));
  }

  return keyValues;
};

const part1 = (instructions: string[]) => {
  let MONAD: number[] = [];
  let stack: IntTriplet[] = [];
  getKV(instructions).forEach((kv) => {
    if (kv.x >= 10) {
      stack.unshift(kv);
    } else {
      let currentIdx = kv.idx;
      let prev = stack.shift()!;

      if (prev.y + kv.x >= 0) {
        MONAD[currentIdx] = 9;
        MONAD[prev.idx] = MONAD[currentIdx] - (prev.y + kv.x);
      } else {
        MONAD[prev.idx] = 9;
        MONAD[currentIdx] = MONAD[prev.idx] + (prev.y + kv.x);
      }
    }
  });

  return MONAD;
};

const part2 = (instructions: string[]) => {
  let MONAD: number[] = [];
  let stack: IntTriplet[] = [];
  getKV(instructions).forEach((kv) => {
    if (kv.x >= 10) {
      stack.unshift(kv);
    } else {
      let currentIdx = kv.idx;
      let prev = stack.shift()!;

      if (prev.y + kv.x >= 0) {
        MONAD[prev.idx] = 1;
        MONAD[currentIdx] = MONAD[prev.idx] + (prev.y + kv.x);
      } else {
        MONAD[currentIdx] = 1;
        MONAD[prev.idx] = MONAD[currentIdx] - (prev.y + kv.x);
      }
    }
  });

  return MONAD;
};

let instructions = getInstructions();
let vm = new VM(instructions);

let first = part1(instructions);
console.log(`Day 24 part 1: ${first.join("")}`);
console.log(vm.run(first));

let second = part2(instructions);
console.log(`\n\nDay 24 part 2: ${second.join("")}`);
console.log(vm.run(second));
