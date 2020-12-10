import fs from 'fs';
import chalk from 'chalk';
import { SolverBase } from './SolverBase';

export class Challenge {
  challengeName: string
  solver: SolverBase
  input: string
  partOne: string
  partTwo: string

  constructor (challengeName: string, Solver: typeof SolverBase) {
    this.challengeName = challengeName;
    this.input = fs.readFileSync(`src/${this.challengeName}/in.txt`, 'utf8');
    this.input = this.input.replace(/\r\n/g, '\n').replace(/\r/g, '\n');
    this.solver = new Solver(this.input);

    this.partOne = this.solvePartOne();
    this.partTwo = this.solvePartTwo();
    fs.writeFileSync(`src/${this.challengeName}/out.txt`, `Part 1: ${this.partOne}\nPart 2: ${this.partTwo}`, 'utf8');
  }

  solvePartOne (): string {
    console.time('Part 01');
    const partOne: string = this.solver.partOne();
    console.timeLog('Part 01', chalk.blue(partOne));
    return partOne;
  }

  solvePartTwo (): string {
    console.time('Part 02');
    const partTwo: string = this.solver.partTwo();
    console.timeLog('Part 02', chalk.blue(partTwo));
    return partTwo;
  }
}
