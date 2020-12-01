import {Challenge} from "../Challenge";
import {SolverBase} from "../SolverBase";

export const challenge = new Challenge('1', class Solver extends SolverBase {
  exp: number[] = this.input.split('\n').map(function (x) {
    return parseInt(x, 10);
  });
  partOne() {
    let answer: string = ""
    dance:
    for (let i = 0; i < this.exp.length; i++) {
      for (let j = 0; j < this.exp.length; j++) {
        if (i === j) continue
        if (this.exp[i] + this.exp[j] === 2020) {
          answer = (this.exp[i] * this.exp[j]).toString()
          break dance;
        }
      }
    }
    return answer
  }

  partTwo() {
    let answer: string = ""
    dance:
    for (let i = 0; i < this.exp.length; i++) {
      for (let j = 0; j < this.exp.length; j++) {
        if (i === j) continue
        for (let k = 0; k < this.exp.length; k++) {
          if (j === k || i === k) continue
          if (this.exp[i] + this.exp[j] + this.exp[k] === 2020) {
            answer = (this.exp[i] * this.exp[j] * this.exp[k]).toString()
            break dance;
          }
        }
      }
    }
    return answer
  }
})
