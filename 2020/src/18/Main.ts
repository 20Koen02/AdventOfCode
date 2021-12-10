import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

class InterpreterOne {
  tokens: string[]
  pos: number

  constructor (input: string) {
    this.tokens = input.match(/\d+|\*|\+|\(|\)/g)!;
    this.pos = 0;
  }

  advance (): string {
    this.pos++;
    return this.tokens[this.pos - 1];
  }

  consume (token: string): boolean {
    if (this.pos < this.tokens.length && this.tokens[this.pos] === token) {
      this.advance();
      return true;
    } else {
      return false;
    }
  }

  evalTerm (): number {
    if (this.consume('(')) {
      const subExpr = this.eval();
      this.consume(')');
      return subExpr;
    } else {
      return parseInt(this.advance());
    }
  }

  eval (): number {
    let res = this.evalTerm();
    while (true) {
      if (this.consume('+')) {
        res += this.evalTerm();
      } else if (this.consume('*')) {
        res *= this.evalTerm();
      } else break;
    }
    return res;
  }
}

class InterpreterTwo extends InterpreterOne {
  evalExpr () {
    let res = this.evalTerm();
    while (this.consume('+')) {
      res += this.evalTerm();
    }
    return res;
  }

  eval (): number {
    let res = this.evalExpr();
    while (this.consume('*')) {
      res *= this.evalExpr();
    }
    return res;
  }
}

export const challenge = new Challenge('18', class Solver extends SolverBase {
  expressions: string[] = this.input.split('\n')

  partOne () {
    let sum = 0;
    this.expressions.forEach((expr) => { sum += new InterpreterOne(expr).eval(); });
    return sum.toString();
  }

  partTwo () {
    let sum = 0;
    this.expressions.forEach((expr) => { sum += new InterpreterTwo(expr).eval(); });
    return sum.toString();
  }
});
