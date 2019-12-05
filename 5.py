def sum(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)

    out = accumulator[0] + accumulator[1]
    stack[stack[instructionPointer + 3]] = out

    instructionPointer += 4
    return stack, instructionPointer


def mul(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)

    out = accumulator[0] * accumulator[1]
    stack[stack[instructionPointer + 3]] = out

    instructionPointer += 4
    return stack, instructionPointer


def inp(stack, instructionPointer, parameters):
    out = int(input("Please provide a single integer: "))
    stack[stack[instructionPointer + 1]] = out

    instructionPointer += 2
    return stack, instructionPointer


def out(stack, instructionPointer, parameters):
    if parameters[0] == 1:
        print(stack[instructionPointer + 1])
    else:
        print(stack[stack[instructionPointer + 1]])

    instructionPointer += 2
    return stack, instructionPointer


def jit(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    if accumulator[0] != 0:
        instructionPointer = accumulator[1]
    else:
        instructionPointer += 3

    return stack, instructionPointer


def jif(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    if accumulator[0] == 0:
        instructionPointer = accumulator[1]
    else:
        instructionPointer += 3

    return stack, instructionPointer


def lss(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    if accumulator[0] < accumulator[1]:
        stack[stack[instructionPointer + 3]] = 1
    else:
        stack[stack[instructionPointer + 3]] = 0

    instructionPointer += 4
    return stack, instructionPointer


def eql(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    if accumulator[0] == accumulator[1]:
        stack[stack[instructionPointer + 3]] = 1
    else:
        stack[stack[instructionPointer + 3]] = 0

    instructionPointer += 4
    return stack, instructionPointer


def main():
    stack = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
    stack = list(map(int, stack.split(",")))
    instructionPointer = 0

    opcodes = {
        1: sum, 
        2: mul,
        3: inp,
        4: out,
        5: jit,
        6: jif,
        7: lss,
        8: eql
    }

    while True:
        if stack[instructionPointer] == 99:
            break

        instruction = [int(x) for x in str(stack[instructionPointer])]
        instruction = [0]*(2 - len(instruction)) + instruction
        op = stack[instructionPointer]
        parameters = [0, 0, 0]
        if len(instruction) > 2:
            op = int(str(instruction[-2]) + str(instruction[-1]))
            parameters = instruction.copy()
            del parameters[-2:]
            parameters.reverse()
            parameters = parameters + [0]*(2 - len(parameters))

        stack, instructionPointer = opcodes.get(op)(stack, instructionPointer, parameters)


def getAccumulator(stack, instructionPointer, parameters):
    accumulator = []
    for x in range(len(parameters)):
        if parameters[x] == 0:
            accumulator.append(stack[stack[instructionPointer + x + 1]])
        elif parameters[x] == 1:
            accumulator.append(stack[instructionPointer + x + 1])
    return accumulator


if __name__ == "__main__":
    main()


