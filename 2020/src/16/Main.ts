import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

interface Range {
  min: number;
  max: number;
}
interface Ticket {
  fields: Map<string, Range[]>;
  myTicket: number[];
  nearbyTickets: number[][];
}
interface Field {
  index: number;
  value: number;
  name: string;
}
interface UnknownField {
  index: number;
  value: number;
  possibleNames: string[];
  name?: string;
}

export const challenge = new Challenge('16', class Solver extends SolverBase {
  parse (): Ticket {
    const lines = this.input.split('\n').filter(Boolean);
    const fields = new Map<string, Range[]>();
    let myTicket: number[] = [];
    const nearbyTickets: number[][] = [];
    let section: string | null = null;

    for (const line of lines) {
      if (line.endsWith(':')) {
        section = line;
        continue;
      }
      if (section === 'your ticket:') {
        myTicket = line.split(',').map((v) => parseInt(v, 10));
      } else if (section === 'nearby tickets:') {
        nearbyTickets.push(line.split(',').map((v) => parseInt(v, 10)));
      } else {
        const [category, value] = line.split(': ');
        const ranges = value.split(' or ').map((rawRange) => {
          const [low, high] = rawRange.split('-');
          return { min: parseInt(low, 10), max: parseInt(high) } as Range;
        });
        fields.set(category, ranges);
      }
    }
    return { fields, myTicket, nearbyTickets };
  }

  parsed: Ticket = this.parse()

  clarifyFields (ticket: number[], otherTickets: number[][], fieldRanges: Map<string, Range[]>): Field[] {
    const categoryRangeEntries = [...fieldRanges.entries()];
    const myFields = ticket.map((value, index) => {
      const otherTicketsFields = otherTickets.map((ticket) => ticket[index]);
      const possibleNames = categoryRangeEntries
        .filter(([, ranges]) =>
          otherTicketsFields.every((ticketField) =>
            ranges.some((range) => range.max >= ticketField && range.min <= ticketField)
          )
        )
        .map((p) => p[0]);
      return { index, value, possibleNames } as UnknownField;
    });

    const certainFields = new Set<string>();
    while (myFields.some((field) => field.possibleNames.length)) {
      for (const myField of myFields.filter((field) => field.possibleNames.length === 1)) {
        myField.name = myField.possibleNames[0];
        certainFields.add(myField.name);
      }
      for (const field of myFields) {
        field.possibleNames = field.possibleNames.filter(
          (possibleName) => !certainFields.has(possibleName)
        );
      }
    }

    return myFields as Field[];
  };

  partOne () {
    const { fields, nearbyTickets }: Ticket = this.parsed;
    return nearbyTickets.flatMap((ticket) => {
      const ranges = [...fields.values()];
      return ticket.filter((field) => !ranges.some((ranges) =>
        ranges.some((range) => range.max >= field && range.min <= field)
      ));
    })
      .reduce((a: number, b: number): number => a + b)
      .toString();
  }

  partTwo () {
    const { myTicket, fields, nearbyTickets } = this.parsed;
    const validTickets = nearbyTickets.filter(
      (ticket) => {
        const ranges = [...fields.values()];
        return ticket.filter((field) => !ranges.some((ranges) =>
          ranges.some((range) => range.max >= field && range.min <= field)
        )).length === 0;
      }
    );
    return this.clarifyFields(myTicket, validTickets, fields)
      .filter((p) => {
        return p.name?.startsWith('departure');
      })
      .map((p) => p.value)
      .reduce((a: number, b: number): number => a * b, 1)
      .toString();
  }
});
