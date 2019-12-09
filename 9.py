def sum(stack, instructionPointer, parameters, relBase):
    accumulator = getAccumulator(stack, instructionPointer, parameters, relBase)

    out = accumulator[0] + accumulator[1]
    stack[stack[instructionPointer + 3]] = out

    instructionPointer += 4
    return stack, instructionPointer, relBase


def mul(stack, instructionPointer, parameters, relBase):
    accumulator = getAccumulator(stack, instructionPointer, parameters, relBase)

    out = accumulator[0] * accumulator[1]
    stack[stack[instructionPointer + 3]] = out

    instructionPointer += 4
    return stack, instructionPointer, relBase


def inp(stack, instructionPointer, parameters, relBase):
    out = int(input("Please provide a single integer: "))
    stack[stack[instructionPointer + 1]] = out

    instructionPointer += 2
    return stack, instructionPointer, relBase


def out(stack, instructionPointer, parameters, relBase):
    if parameters[0] == 1:
        print(stack[instructionPointer + 1])
    else:
        print(stack[stack[instructionPointer + 1]])

    instructionPointer += 2
    return stack, instructionPointer, relBase


def jit(stack, instructionPointer, parameters, relBase):
    accumulator = getAccumulator(stack, instructionPointer, parameters, relBase)
    
    if accumulator[0] != 0:
        instructionPointer = accumulator[1]
    else:
        instructionPointer += 3

    return stack, instructionPointer, relBase


def jif(stack, instructionPointer, parameters, relBase):
    accumulator = getAccumulator(stack, instructionPointer, parameters, relBase)
    
    if accumulator[0] == 0:
        instructionPointer = accumulator[1]
    else:
        instructionPointer += 3

    return stack, instructionPointer, relBase


def lss(stack, instructionPointer, parameters, relBase):
    accumulator = getAccumulator(stack, instructionPointer, parameters, relBase)
    
    if accumulator[0] < accumulator[1]:
        stack[stack[instructionPointer + 3]] = 1
    else:
        stack[stack[instructionPointer + 3]] = 0

    instructionPointer += 4
    return stack, instructionPointer, relBase


def eql(stack, instructionPointer, parameters, relBase):
    accumulator = getAccumulator(stack, instructionPointer, parameters, relBase)
    
    if accumulator[0] == accumulator[1]:
        stack[stack[instructionPointer + 3]] = 1
    else:
        stack[stack[instructionPointer + 3]] = 0

    instructionPointer += 4
    return stack, instructionPointer, relBase

def rel(stack, instructionPointer, parameters, relBase):
    accumulator = getAccumulator(stack, instructionPointer, parameters, relBase)
    relBase = relBase + accumulator[0]

    instructionPointer += 2
    return stack, instructionPointer, relBase


def main():
    stack = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
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
        8: eql,
        9: rel
    }

    relBase = 0

    while True:
        if stack[instructionPointer] == 99:
            break

        instruction = [int(x) for x in str(stack[instructionPointer])]
        instruction = [0]*(2 - len(instruction)) + instruction

        op = int(str(instruction[-2]) + str(instruction[-1]))
        
        parameters = instruction.copy()
        del parameters[-2:]
        parameters.reverse()
        parameters = parameters + [0]*(2 - len(parameters))

        stack, instructionPointer, relBase = opcodes.get(op)(stack, instructionPointer, parameters, relBase)


def getAccumulator(stack, instructionPointer, parameters, relBase):
    accumulator = []
    for x in range(len(parameters)):
        if parameters[x] == 0:
            accumulator.append(stack[stack[instructionPointer + x + 1]])
        elif parameters[x] == 1:
            accumulator.append(stack[instructionPointer + x + 1])
        elif parameters[x] == 2:
            accumulator.append(stack[stack[instructionPointer + x + 1] + relBase])
    return accumulator


if __name__ == "__main__":
    main()

