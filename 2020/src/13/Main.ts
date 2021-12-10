import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

type Bus = [busId: number, index: number];

export const challenge = new Challenge('13', class Solver extends SolverBase {
  lines: string[] = this.input.split('\n')
  buses: Bus[] = this.parseBuses(this.lines[1]);
  earliest: number = parseInt(this.lines[0]);

  parseBuses (line: string) {
    return line.split(',').map((b, bi) => [+b, bi] as Bus).filter(([b]) => !isNaN(b));
  };

  partOne () {
    const [earliestBus, waitTime] = this.buses.reduce(
      ([cur, earliest], [busId]) => {
        const interval = this.earliest % busId;
        const wait = busId - interval;
        return earliest > wait ? [busId, wait] : [cur, earliest];
      },
      [0, Number.MAX_SAFE_INTEGER]
    );
    return (earliestBus * waitTime).toString();
  }

  partTwo () {
    let t = this.buses[0][0];
    let skip = t;
    for (let i = 1; i < this.buses.length; i++) {
      const [busId, index] = this.buses[i];
      while ((t + index) % busId !== 0) t += skip;
      skip = skip * busId;
    }
    return t.toString();
  }
});
