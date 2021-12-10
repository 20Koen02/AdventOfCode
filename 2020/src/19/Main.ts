import { Challenge } from '../Challenge';
import { SolverBase } from '../SolverBase';
import assert from 'assert';

type RuleVal = string | string[][]
type RuleMap = Map<string, RuleVal>

export const challenge = new Challenge('19', class Solver extends SolverBase {
  parts: string[] = this.input.split('\n\n')
  rulesRaw: string[] = this.parts[0].split('\n')
  messages: string[] = this.parts[1].split('\n')

  parse (rulesRaw: string[]): RuleMap {
    return rulesRaw.map(r => r.split(': ')).reduce((m, [n, r]) => {
      return m.set(n, r[0] === '"' ? r[1] : r.split(' | ').map(n => n.split(' ')));
    }, new Map());
  }

  msgMatchRules (msg: RuleVal, [rule, ...rest]: any[], rules: RuleMap): boolean {
    if (!rule) return !msg;
    const next: RuleVal = rules.get(rule)!;
    if (next instanceof Array) {
      return next.some(r => this.msgMatchRules(msg, r.concat(rest), rules));
    } else {
      return msg[0] === next && this.msgMatchRules(msg.slice(1), rest, rules);
    }
  };

  solve (rulesRaw: string[], messages: string[]): number {
    const rules = this.parse(rulesRaw);
    const firstRule = rules.get('0')!;
    assert(Array.isArray(firstRule));
    return messages.map((msg: string) => {
      return firstRule.some((r: string[]) => this.msgMatchRules(msg, r, rules));
    }).filter(Boolean).length;
  }

  partOne () {
    return this.solve(this.rulesRaw, this.messages).toString();
  }

  partTwo () {
    const rulesRaw: string[] = this.rulesRaw.concat(['8: 42 | 42 8', '11: 42 31 | 42 11 31']);
    return this.solve(rulesRaw, this.messages).toString();
  }
});
