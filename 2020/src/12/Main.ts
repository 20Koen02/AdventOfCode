import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

interface Instruction {
  op: string;
  val: number;
}

interface Location {
  f?: number
  sx: number,
  sy: number,
  wx?: number,
  wy?: number,
}

export const challenge = new Challenge('12', class Solver extends SolverBase {
  instructions: Instruction[] = this.input.split('\n').map((line): Instruction => {
    return {
      op: line.slice(0, 1),
      val: parseInt(line.slice(1))
    };
  })

  mod (n: number, m: number) {
    return ((n % m) + m) % m;
  }

  rotate (dir: Location, ins: Instruction): Location {
    const rot: number = ins.op === 'R' ? ins.val : 360 - ins.val;
    const curWy = dir.wy!;
    switch (rot) {
      case 90:
        dir.wy = dir.wx! * -1;
        dir.wx = curWy;
        break;
      case 270:
        dir.wy = dir.wx!;
        dir.wx = curWy * -1;
        break;
      case 180:
        dir.wx! *= -1;
        dir.wy! *= -1;
        break;
    }
    return dir;
  }

  partOne () {
    const dir: Location = {
      f: 90,
      sx: 0,
      sy: 0
    };
    for (let i = 0; i < this.instructions.length; i++) {
      const ins = this.instructions[i];
      switch (ins.op) {
        case 'N':
          dir.sy += ins.val;
          break;
        case 'E':
          dir.sx += ins.val;
          break;
        case 'S':
          dir.sy -= ins.val;
          break;
        case 'W':
          dir.sx -= ins.val;
          break;
        case 'L':
          dir.f = this.mod(dir.f! - ins.val, 360);
          break;
        case 'R':
          dir.f = this.mod(dir.f! + ins.val, 360);
          break;
        case 'F':
          switch (dir.f) {
            case 0:
              dir.sy += ins.val;
              break;
            case 90:
              dir.sx += ins.val;
              break;
            case 180:
              dir.sy -= ins.val;
              break;
            case 270:
              dir.sx -= ins.val;
              break;
          }
          break;
      }
    }
    return (Math.abs(dir.sx) + Math.abs(dir.sy)).toString();
  }

  partTwo () {
    let dir: Location = {
      sx: 0,
      sy: 0,
      wx: 10,
      wy: 1
    };
    for (let i = 0; i < this.instructions.length; i++) {
      const ins = this.instructions[i];
      switch (ins.op) {
        case 'R':
        case 'L':
          dir = this.rotate(dir, ins);
          break;
        case 'N':
          dir.wy! += ins.val;
          break;
        case 'E':
          dir.wx! += ins.val;
          break;
        case 'S':
          dir.wy! -= ins.val;
          break;
        case 'W':
          dir.wx! -= ins.val;
          break;
        case 'F':
          dir.sx += dir.wx! * ins.val;
          dir.sy += dir.wy! * ins.val;
          break;
      }
    }
    return (Math.abs(dir.sx) + Math.abs(dir.sy)).toString();
  }
});
