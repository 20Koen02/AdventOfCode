import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('17', class Solver extends SolverBase {
  lines: number[][] = this.input.split('\n').map((l) => l.split('').map(char => char === '#' ? 1 : 0))
  STR = 6 + Math.max(this.lines.length, this.lines[0].length);
  W = this.STR * this.STR * this.STR * 8;
  X = this.STR * this.STR * 4;
  Y = this.STR * 2;

  partOne () {
    const input = this.lines;
    let activeCubes: Set<number> = new Set();
    for (let y = 0; y < input.length; y++) {
      const row = input[y];
      for (let x = 0; x < row.length; x++) {
        if (row[x]) activeCubes.add(this.XYZ(x, y, 0));
      }
    }
    let nextCubes: Set<number> = new Set();
    const countNeighbors = new Map();
    for (let n = 0; n < 6; n++) {
      countNeighbors.clear();
      for (const xyz of activeCubes.values()) {
        const [x, y, z] = this.getXYZ(xyz);
        for (let i = -1; i <= 1; i++) {
          for (let j = -1; j <= 1; j++) {
            for (let k = -1; k <= 1; k++) {
              const xyzN = this.XYZ(x + i, y + j, z + k);
              countNeighbors.set(xyzN, (countNeighbors.get(xyzN) | 0) + 1);
            }
          }
        }
      }
      nextCubes.clear();
      for (const [xyz, totalNeighbors] of countNeighbors) {
        if (activeCubes.has(xyz)) {
          if (totalNeighbors === 3 || totalNeighbors === 4) {
            nextCubes.add(xyz);
          }
        } else if (totalNeighbors === 3) {
          nextCubes.add(xyz);
        }
      }
      [activeCubes, nextCubes] = [nextCubes, activeCubes];
    }
    return activeCubes.size.toString();
  }

  partTwo () {
    const input = this.lines;
    let activeCubes: Set<number> = new Set();
    for (let y = 0; y < input.length; y++) {
      const row = input[y];
      for (let x = 0; x < row.length; x++) {
        if (row[x]) activeCubes.add(this.WXYZ(0, x, y, 0));
      }
    }
    let nextCubes: Set<number> = new Set();
    const countNeighbors = new Map();
    for (let n = 0; n < 6; n++) {
      countNeighbors.clear();
      for (const wxyz of activeCubes.values()) {
        const [w, x, y, z] = this.getWXYZ(wxyz);
        for (let h = -1; h <= 1; h++) {
          for (let i = -1; i <= 1; i++) {
            for (let j = -1; j <= 1; j++) {
              for (let k = -1; k <= 1; k++) {
                const wxyzN = this.WXYZ(w + h, x + i, y + j, z + k);
                countNeighbors.set(wxyzN, (countNeighbors.get(wxyzN) | 0) + 1);
              }
            }
          }
        }
      }
      nextCubes.clear();
      for (const [wxyz, totalNeighbors] of countNeighbors) {
        if (activeCubes.has(wxyz)) {
          if (totalNeighbors === 3 || totalNeighbors === 4) {
            nextCubes.add(wxyz);
          }
        } else if (totalNeighbors === 3) {
          nextCubes.add(wxyz);
        }
      }
      [activeCubes, nextCubes] = [nextCubes, activeCubes];
    }
    return activeCubes.size.toString();
  }

  XYZ (x: number, y: number, z: number) {
    return (x + this.STR) * this.X + (y + this.STR) * this.Y + (z + this.STR);
  };

  WXYZ (w: number, x: number, y: number, z: number) {
    return (w + this.STR) * this.W + (x + this.STR) * this.X + (y + this.STR) * this.Y + (z + this.STR);
  }

  getXYZ (xyz: number) {
    const x = (xyz / this.X) | 0;
    xyz -= x * this.X;
    const y = (xyz / this.Y) | 0;
    xyz -= y * this.Y;
    const z = xyz - this.STR;
    return [x - this.STR, y - this.STR, z];
  };

  getWXYZ (wxyz: number) {
    const w = (wxyz / this.W) | 0;
    wxyz -= w * this.W;
    const x = (wxyz / this.X) | 0;
    wxyz -= x * this.X;
    const y = (wxyz / this.Y) | 0;
    wxyz -= y * this.Y;
    const z = wxyz - this.STR;
    return [w - this.STR, x - this.STR, y - this.STR, z];
  };
});
