import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

interface passwordObject {
  min: number,
  max: number,
  letter: string,
  password: string
}

export const challenge = new Challenge('2', class Solver extends SolverBase {
  lines: passwordObject[] = this.input.split('\n').map(function (x): passwordObject {
    const splitLines: string[] = x.split(' ');
    const minMax: string[] = splitLines[0].split('-');
    return {
      min: parseInt(minMax[0]),
      max: parseInt(minMax[1]),
      letter: splitLines[1][0],
      password: splitLines[2].replace(/\r/g, '')
    };
  });

  partOne () {
    let counter: number = 0;
    for (const i in this.lines) {
      const line: passwordObject = this.lines[i];
      const matcher: RegExp = new RegExp(line.letter, 'g');
      const length: number = (line.password.match(matcher) || []).length;
      if (length >= line.min && length <= line.max) counter++;
    }
    return counter.toString();
  }

  partTwo () {
    let counter: number = 0;
    for (const i in this.lines) {
      const line: passwordObject = this.lines[i];
      const firstContains: boolean = line.password[line.min - 1] === line.letter;
      const secondContains: boolean = line.password[line.max - 1] === line.letter;
      if ((firstContains || secondContains) && !(firstContains && secondContains)) counter++;
    }
    return counter.toString();
  }
});
