import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('06', class Solver extends SolverBase {
  lines: string[] = this.input.split('\n\n')

  partOne () {
    let res: number = 0;
    this.lines.forEach((group) => {
      res += new Set(group.replace(/\n/g, '').split('')).size;
    });
    return res.toString();
  }

  partTwo () {
    let res: number = 0;
    this.lines.forEach((group) => {
      const countMap: Map<string, number> = group
        .replace(/\n/g, '')
        .split('')
        .reduce((a: Map<string, number>, c: string) => a.set(c, (a.get(c) || 0) + 1), new Map());
      countMap.forEach((char: number) => {
        res += char === group.split('\n').length ? 1 : 0;
      });
    });
    return res.toString();
  }
});
