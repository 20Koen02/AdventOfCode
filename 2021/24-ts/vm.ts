type Opcode = "inp" | "add" | "mul" | "div" | "mod" | "eql";
type Operand = "w" | "x" | "y" | "z";

class Instruction {
  opcode: Opcode;
  operand1: Operand;
  operand2: Operand | Number;

  constructor(instr: string) {
    const [opcode, ...operands] = instr.trim().split(" ");
    this.opcode = opcode as Opcode;
    this.operand1 = operands[0] as Operand;
    this.operand2 = isNaN(+operands[1])
      ? (operands[1] as Operand)
      : +operands[1];
  }
}

export class VM {
  acc = {
    w: 0,
    x: 0,
    y: 0,
    z: 0,
  };
  program: Instruction[];

  constructor(program: string[]) {
    this.program = program.map((instr) => new Instruction(instr))
  }

  run(inputs: number[]) {
    for (const instr of this.program) {
      switch (instr.opcode) {
        case "inp":
          this.acc[instr.operand1] = inputs.shift()!;
          break;
        case "add":
          this.acc[instr.operand1] += this.getOperand(instr.operand2);
          break;
        case "mul":
          this.acc[instr.operand1] *= this.getOperand(instr.operand2);
          break;
        case "div":
          this.acc[instr.operand1] = Math.floor(
            this.getOperand(instr.operand1) / this.getOperand(instr.operand2)
          );
          break;
        case "mod":
          this.acc[instr.operand1] %= this.getOperand(instr.operand2);
          break;
        case "eql":
          this.acc[instr.operand1] = +(
            this.acc[instr.operand1] === this.getOperand(instr.operand2)
          );
          break;
      }
    }
    return this.acc;
  }

  private getOperand(operand: Operand | Number) {
    return typeof operand === "number" ? operand : this.acc[operand as Operand];
  }
}
