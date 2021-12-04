import fs from "fs";

class Bingo {
  private numbers: number[] = [];
  private boards: number[][][] = [];
  private lastDrawn: number = 0;
  private wins: Set<number> = new Set();

  private reset(): void {
    const parts = readParts();
    this.numbers = parts.shift()!.split(",").map(Number);
    this.boards = parts.map((board) =>
      board.split("\n").map((row) => row.trim().split(/\s+/g).map(Number))
    );
  }

  private checkWins(checkAll: boolean = false): number {
    for (const [idx, board] of this.boards.entries()) {
      const horizontal = board.some((row) => row.every((cell) => cell === -1));
      const vertical = transpose(board).some((row) =>
        row.every((cell) => cell === -1)
      );

      if (horizontal || vertical) {
        const sumUnmarked = ([] as number[])
          .concat(...board)
          .filter((cell) => cell !== -1)
          .reduce((a, b) => a + b);

        if (checkAll) this.wins.add(idx);

        if (!checkAll || this.wins.size === this.boards.length) {
          return sumUnmarked * this.lastDrawn;
        }
      }
    }
    return -1;
  }

  private markBoards(number: number): void {
    this.boards = this.boards.map((board) =>
      board.map((row) => row.map((cell) => (cell === number ? -1 : cell)))
    );
  }

  private drawNumber(): number {
    this.lastDrawn = this.numbers.shift()!;
    return this.lastDrawn;
  }

  public solve(lastWin: boolean): number {
    this.reset();

    let result = -1;
    while (result < 0) {
      this.markBoards(this.drawNumber());
      result = this.checkWins(lastWin);
    }
    return result;
  }
}

const transpose = (m: number[][]) => m[0].map((x, i) => m.map((x) => x[i]));
const readParts = () => fs.readFileSync(`in.txt`, "utf8").split("\n\n");

const bingo = new Bingo();
console.log(`Day 4 part one: ${bingo.solve(false)}`);
console.log(`Day 4 part two: ${bingo.solve(true)}`);
