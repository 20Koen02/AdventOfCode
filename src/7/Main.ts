import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('7', class Solver extends SolverBase {
  bags: string[] = this.input.split('\n')

  partOneContainsGoldBag: Set<string> = new Set<string>()
  partTwoCount: number = -1

  partOneRecursive (bagSearch: string): void {
    const bagContainsMatcher = new RegExp(`(\\d) ${bagSearch}`);

    this.bags.forEach((bag) => {
      if (bag.match(bagContainsMatcher)) {
        const bagFiltered = bag.split(' ').splice(0, 2).join(' ');
        this.partOneContainsGoldBag.add(bagFiltered);
        this.partOneRecursive(bagFiltered);
      }
    });
  }

  partTwoRecursive (bagSearch: string, amount: number): void {
    this.partTwoCount += amount;
    const findBagMatcher = new RegExp(`^${bagSearch}`);
    const bagMatcher = /(\d).+?(?= bag)/g;

    this.bags.forEach((bag) => {
      if (bag.match(findBagMatcher)) {
        const containsBags = bag.match(bagMatcher);
        if (containsBags) {
          for (const containsBag of containsBags) {
            const split = containsBag.split(' ');
            const num = parseInt(split[0]);
            split.shift();
            this.partTwoRecursive(split.join(' '), num * amount);
          }
        }
      }
    });
  }

  partOne () {
    this.partOneRecursive('shiny gold');
    return this.partOneContainsGoldBag.size.toString();
  }

  partTwo () {
    this.partTwoRecursive('shiny gold', 1);
    return this.partTwoCount.toString();
  }
});
