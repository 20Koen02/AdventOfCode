import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

interface Instruction {
  op: string
  arg: number
}

export const challenge = new Challenge('08', class Solver extends SolverBase {
  ops: Instruction[] = this.input.split('\n').map((opLines) => {
    const splitOpLines = opLines.split(' ');
    const instruction: Instruction = {
      op: splitOpLines[0],
      arg: parseInt(splitOpLines[1])
    };
    return instruction;
  })

  res: number = 0

  execute (ops: Instruction[]): number {
    const visited: number[] = [];
    let accumulator: number = 0;
    let pointer: number = 0;

    let exitCode: number = 1;
    let halt: boolean = false;

    while (!halt) {
      visited.push(pointer);

      switch (ops[pointer].op) {
        case 'acc':
          accumulator += ops[pointer].arg;
          pointer++;
          break;
        case 'jmp':
          pointer += ops[pointer].arg;
          break;
        case 'nop':
          pointer++;
          break;
      }

      if (pointer >= ops.length) {
        this.res = accumulator;
        exitCode = 0; // Exit code 0 - Program finished
        halt = true;
      }
      if (visited.includes(pointer)) {
        this.res = accumulator;
        exitCode = 1; // Exit code 01 - Loop detected
        halt = true;
      }
    }
    return exitCode;
  }

  partOne () {
    this.execute(this.ops);
    return this.res.toString();
  }

  partTwo () {
    for (let i = 0; i < this.ops.length; i++) {
      if (this.ops[i].op !== 'jmp' && this.ops[i].op !== 'nop') continue;
      const opsCopy = JSON.parse(JSON.stringify(this.ops));
      opsCopy[i].op = opsCopy[i].op === 'jmp' ? 'nop' : 'jmp';
      if (this.execute(opsCopy) === 0) break;
    }
    return this.res.toString();
  }
});
