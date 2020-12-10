import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('05', class Solver extends SolverBase {
  tickets: string[] = this.input.split('\n');

  seats: number[] = this.tickets.map((ticket): number => {
    const bin: string = ticket.replace(/[BR]/g, '1').replace(/[FL]/g, '0');
    return parseInt(bin, 2);
  })

  partOne () {
    return Math.max(...this.seats).toString();
  }

  partTwo () {
    let res: number = 0;
    this.seats.forEach((seat) => {
      if (this.seats.includes(seat + 2) && !this.seats.includes(seat + 1)) res = seat + 1;
    });
    return res.toString();
  }
});
