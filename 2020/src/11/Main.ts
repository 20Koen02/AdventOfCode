import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

interface Neighbors {
  empty: number,
  occupied: number
}

export const challenge = new Challenge('11', class Solver extends SolverBase {
  table: String[] = this.input.split('\n')
  search = [
    [-1, 0],
    [-1, -1],
    [-1, 1],
    [0, 1],
    [0, -1],
    [1, 0],
    [1, -1],
    [1, 1]
  ];

  getOccupiedSeats (neighCallback: (table: String[], x: number, y: number, search: number[][]) => Neighbors, maxOcc: number): number {
    let prevTable: String[] = [];
    const newTable: String[] = [...this.table];

    while (prevTable.toString() !== newTable.toString()) {
      prevTable = [...newTable];
      for (let x = 0; x < prevTable.length; x++) {
        for (let y = 0; y < prevTable[x].length; y++) {
          if (prevTable[x][y] === '.') continue;
          const neigh: Neighbors = neighCallback(prevTable, x, y, this.search);
          if (prevTable[x][y] === 'L' && neigh.occupied === 0) {
            newTable[x] = newTable[x].substr(0, y) + '#' + newTable[x].substr(y + '#'.length);
          } else if (prevTable[x][y] === '#' && neigh.occupied >= maxOcc) {
            newTable[x] = newTable[x].substr(0, y) + 'L' + newTable[x].substr(y + 'L'.length);
          }
        }
      }
    }
    return (newTable.join().match(/#/g)!).length;
  }

  partOneNeighbor (table: String[], x: number, y: number, search: number[][]): Neighbors {
    const nArray: String[] = search
      .filter(([h, j]) => h + x >= 0 && h + x < table.length && j + y >= 0 && j + y < table[0].length)
      .map(([h, j]) => table[h + x][j + y]);
    return {
      empty: nArray.reduce((n, x) => n + (x === 'L' ? 1 : 0), 0),
      occupied: nArray.reduce((n, x) => n + (x === '#' ? 1 : 0), 0)
    };
  }

  partTwoNeighbor (table: String[], x: number, y:number, search: number[][]): Neighbors {
    const neigh: Neighbors = {
      empty: 0,
      occupied: 0
    };
    search.forEach((scale) => {
      let c = 1;
      while (true) {
        if (!table[x + scale[0] * c]) break;
        if (!table[x + scale[0] * c][y + scale[1] * c]) break;
        const char = table[x + scale[0] * c][y + scale[1] * c];
        if (char === 'L') {
          neigh.empty++;
          break;
        } else if (char === '#') {
          neigh.occupied++;
          break;
        } else c++;
      }
    });
    return neigh;
  }

  partOne () {
    return this.getOccupiedSeats(this.partOneNeighbor, 4).toString();
  }

  partTwo () {
    return this.getOccupiedSeats(this.partTwoNeighbor, 5).toString();
  }
});
