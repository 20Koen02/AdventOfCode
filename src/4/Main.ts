import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';

export const challenge = new Challenge('4', class Solver extends SolverBase {
  reqFields: ReadonlyArray<string> = [
    'byr',
    'iyr',
    'eyr',
    'hgt',
    'hcl',
    'ecl',
    'pid'
  ];

  passports = this.input.split('\n\n').map((x) => {
    return x.replace(/\n/g, ' ').split(' ').reduce((acc, curr) => {
      const [key, value] = curr.split(':');
      return {
        ...acc,
        [key]: value
      };
    }, {} as Record<string, string>);
  });

  range = (value: string, min: number, max: number): boolean => {
    const num = parseInt(value, 10);
    return num >= min && num <= max;
  };

  partOne () {
    const validPassports = this.passports.reduce((acc, curr) => {
      const keys = Object.keys(curr);
      const valid = this.reqFields.every((o) => keys.includes(o));
      return valid ? acc + 1 : acc;
    }, 0);

    return validPassports.toString();
  }

  partTwo () {
    const validPassports = this.passports.reduce((acc, curr) => {
      const keys = Object.keys(curr);
      const containsReqFields = this.reqFields.every((o) =>
        keys.includes(o)
      );
      let validField = true;

      for (const [key, value] of Object.entries(curr)) {
        switch (key) {
          case 'byr':
            validField = this.range(value, 1920, 2002);
            break;
          case 'iyr':
            validField = this.range(value, 2010, 2020);
            break;
          case 'eyr':
            validField = this.range(value, 2020, 2030);
            break;
          case 'hgt': {
            if (!/(\d+)(cm|in)/.test(value)) {
              validField = false;
              break;
            }
            const height = value.replace(/[cmin]/g, '');
            validField = value.includes('cm')
              ? this.range(height, 150, 193)
              : this.range(height, 59, 76);
            break;
          }
          case 'hcl':
            validField = /#[a-f0-9]{6}/.test(value);
            break;
          case 'ecl':
            validField = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].some(
              (o) => value === o
            );
            break;
          case 'pid':
            validField = /^\d{9}$/.test(value);
            break;
        }

        if (!validField) {
          break;
        }
      }
      return containsReqFields && validField ? acc + 1 : acc;
    }, 0);

    return validPassports.toString();
  }
});
