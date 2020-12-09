import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('9', class Solver extends SolverBase {
  nums: number[] = this.input.split('\n').map((line) => {
    return parseInt(line);
  })

  partOne () {
    let ans: number = 0;
    for (let i = 25; i < this.nums.length; i++) {
      const preamble = this.nums.slice(i - 25, i);
      if (preamble.some((x) => preamble.includes(this.nums[i] - x))) continue;
      ans = this.nums[i];
      break;
    }
    return ans.toString();
  }

  partTwo () {
    const invalid: number = parseInt(this.partOne());
    let ans: number = 0;

    dance:
    for (let i = 0; i < this.nums.length; i++) {
      for (let j = i + 2; j < this.nums.length; j++) {
        const contiguousArray = this.nums.slice(i, j);
        if (contiguousArray.reduce((a, b) => a + b, 0) === invalid) {
          ans = Math.min(...contiguousArray) + Math.max(...contiguousArray);
          break dance;
        }
      }
    }
    return ans.toString();
  }
});
