import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('1', class Solver extends SolverBase {
  exp: number[] = this.input.split('\n').map(function (x) {
    return parseInt(x, 10);
  });

  partOne () {
    let answer: string = '';
    for (let i = 0; i < this.exp.length; i++) {
      const num2: number = 2020 - this.exp[i];
      if (this.exp.includes(num2)) {
        answer = (this.exp[i] * num2).toString();
        break;
      }
    }
    return answer;
  }

  partTwo () {
    let answer: string = '';
    dance:
    for (let i = 0; i < this.exp.length; i++) {
      const num2: number = 2020 - this.exp[i];
      for (let j = 0; j < this.exp.length; j++) {
        const num3: number = num2 - this.exp[j];
        if (this.exp.includes(num3)) {
          answer = (this.exp[i] * this.exp[j] * num3).toString();
          break dance;
        }
      }
    }
    return answer;
  }
});
