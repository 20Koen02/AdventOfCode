import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

interface Instruction {
  type: 'mask' | 'mem',
  pos?: number,
  val: string
}

export const challenge = new Challenge('14', class Solver extends SolverBase {
  lines: string[] = this.input.split('\n');
  instructions: Instruction[] = []

  constructor (input: string) {
    super(input);
    this.lines.forEach((line) => {
      if (line.includes('mem')) {
        const digits = line.match(/\d+/g)!;
        this.instructions.push({
          type: 'mem',
          pos: parseInt(digits[0]),
          val: digits[1]
        });
      } else {
        this.instructions.push({
          type: 'mask',
          val: line.match(/[\dX]{36}/)![0]
        });
      }
    });
  }

  toBinString (v: number): string {
    return (v >>> 0).toString(2);
  }

  partOne () {
    const mem: Map<number, number> = new Map<number, number>();
    let curMask: Map<number, number> = new Map<number, number>();
    for (let x = 0; x < this.instructions.length; x++) {
      if (this.instructions[x].type === 'mask') {
        curMask = new Map<number, number>();
        const reg = /[10]/g;
        let match;
        while ((match = reg.exec(this.instructions[x].val)) != null) {
          curMask.set(match.index, parseInt(match[0]));
        }
      } else {
        const bin: string[] = this.toBinString(parseInt(this.instructions[x].val)).padStart(36, '0').split('');
        curMask.forEach((val, pos) => {
          bin[pos] = val.toString();
        });
        mem.set(this.instructions[x].pos!, parseInt(bin.join(''), 2));
      }
    }
    let sum = 0;
    mem.forEach((val) => { sum += val; });
    return sum.toString();
  }

  partTwo () {
    let mem: Map<number, number> = new Map<number, number>();
    let curMask: string = '';
    for (let x = 0; x < this.instructions.length; x++) {
      if (this.instructions[x].type === 'mask') {
        curMask = this.instructions[x].val;
      } else {
        const floatingMem: Map<number, number> = new Map<number, number>();
        const posBin: string[] = this.toBinString(this.instructions[x].pos!).padStart(36, '0').split('');
        const mask = curMask.split('');
        const maskIds: number[] = [];

        for (let i = 0; i < mask.length; i++) {
          if (mask[i] === 'X') maskIds.push(i);
          if (mask[i] === '0') continue;
          posBin[i] = mask[i];
        }

        for (let i = 0; i < 2 ** maskIds.length; i++) {
          const curMaskBin = this.toBinString(i).padStart(maskIds.length, '0').split('');
          let j = 0;
          maskIds.forEach((id) => {
            posBin[id] = curMaskBin[j];
            j++;
          });
          floatingMem.set(parseInt(posBin.join(''), 2), parseInt(this.instructions[x].val));
        }
        mem = new Map<number, number>([...mem, ...floatingMem]);
      }
    }
    let sum = 0;
    mem.forEach((val) => { sum += val; });
    return sum.toString();
  }
});
