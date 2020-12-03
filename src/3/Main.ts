import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('3', class Solver extends SolverBase {
  lines: string[] = this.input.split('\n').map(function (x): string {
    return x.trim();
  });

  trees (dx: number, dy: number): number {
    let counter: number = 0;
    for (let x = 0, y = 0; y < this.lines.length; x += dx, y += dy) {
      const line: string = this.lines[y];
      if (line[x % line.length] === '#') counter++;
    }
    return counter;
  }

  partOne () {
    return this.trees(3, 1).toString();
  }

  partTwo () {
    return (this.trees(1, 1) *
      this.trees(3, 1) *
      this.trees(5, 1) *
      this.trees(7, 1) *
      this.trees(1, 2)).toString();
  }
});
