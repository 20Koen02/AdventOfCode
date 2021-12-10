import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('10', class Solver extends SolverBase {
  nums: number[] = this.input.split('\n').map((num) => {
    return parseInt(num);
  }).sort((a, b) => a - b);

  partOne () {
    this.nums.unshift(0);
    let oneJoltCount = 0;
    let threeJoltCount = 1;
    for (let i = 1; i < this.nums.length; i++) {
      const diff = this.nums[i] - this.nums[i - 1];
      switch (diff) {
        case 1:
          oneJoltCount++;
          break;
        case 3:
          threeJoltCount++;
          break;
      }
    }
    return (oneJoltCount * threeJoltCount).toString();
  }

  partTwo () {
    const count: Map<number, number> = new Map<number, number>();
    count.set(0, 1);
    for (let i = 0; i < this.nums.length; i++) {
      let j = i + 1;
      while (this.nums[j] <= this.nums[i] + 3) {
        count.set(j, (count.get(j) || 0) + (count.get(i)!));
        j++;
      }
    }
    return (count.get(this.nums.length - 1)!).toString();
  }
});
