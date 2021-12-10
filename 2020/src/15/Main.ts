import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('15', class Solver extends SolverBase {
  startNums: number[] = this.input.trim().split(',').map((char) => parseInt(char))

  solve (rounds: number): number {
    const nums: Map<number, number> = new Map<number, number>();
    this.startNums.forEach((num, i) => {
      if (i < this.startNums.length - 1) nums.set(num, i);
    });
    let prevTurn: number = this.startNums[this.startNums.length - 1];

    for (let i = this.startNums.length; i < rounds; i++) {
      const lastAdd = prevTurn;
      if (nums.has(prevTurn)) {
        prevTurn = i - 1 - nums.get(prevTurn)!;
        nums.set(lastAdd, i - 1);
      } else {
        nums.set(prevTurn, i - 1);
        prevTurn = 0;
      }
    }
    return prevTurn;
  }

  partOne () {
    return this.solve(2020).toString();
  }

  partTwo () {
    return this.solve(30000000).toString();
  }
});
