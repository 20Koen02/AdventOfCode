import fs from 'fs';
import chalk from "chalk";
import {SolverBase} from "./SolverBase";

export class Challenge {
  challengeName: string
  solver: SolverBase
  input: string

  constructor(challengeName: string, Solver: typeof SolverBase) {
    this.challengeName = challengeName;
    this.input = fs.readFileSync(`src/${this.challengeName}/in.txt`, 'utf8');
    this.solver = new Solver(this.input)

    this.solve()
  }

  solve(): void {
    fs.writeFileSync(`src/${this.challengeName}/out.txt`, `Part 1: ${this.solvePartOne()}\nPart 2: ${this.solvePartTwo()}`, 'utf8')
  }

  solvePartOne(): string {
    console.time('Part 1')
    const partOne: string = this.solver.partOne()
    console.timeLog('Part 1', chalk.blue(partOne))
    return partOne
  }

  solvePartTwo(): string {
    console.time('Part 2')
    const partTwo: string = this.solver.partTwo()
    console.timeLog('Part 2', chalk.blue(partTwo))
    return partTwo
  }
}
