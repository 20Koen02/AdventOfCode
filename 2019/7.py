import itertools

def sum(stack, instructionPointer, parameters, inp):
    accumulator = getAccumulator(stack, instructionPointer, parameters)

    out = accumulator[0] + accumulator[1]
    stack[stack[instructionPointer + 3]] = out

    instructionPointer += 4
    return stack, instructionPointer


def mul(stack, instructionPointer, parameters, inp):
    accumulator = getAccumulator(stack, instructionPointer, parameters)

    out = accumulator[0] * accumulator[1]
    stack[stack[instructionPointer + 3]] = out

    instructionPointer += 4
    return stack, instructionPointer


def inp(stack, instructionPointer, parameters, inp):
    out = inp
    stack[stack[instructionPointer + 1]] = out

    instructionPointer += 2
    return stack, instructionPointer


def out(stack, instructionPointer, parameters, inp):
    if parameters[0] == 1:
        out = stack[instructionPointer + 1]
    else:
        out = stack[stack[instructionPointer + 1]]

    stack[0] = out

    instructionPointer += 2
    return stack, instructionPointer


def jit(stack, instructionPointer, parameters, inp):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    if accumulator[0] != 0:
        instructionPointer = accumulator[1]
    else:
        instructionPointer += 3

    return stack, instructionPointer


def jif(stack, instructionPointer, parameters, inp):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    if accumulator[0] == 0:
        instructionPointer = accumulator[1]
    else:
        instructionPointer += 3

    return stack, instructionPointer


def lss(stack, instructionPointer, parameters, inp):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    if accumulator[0] < accumulator[1]:
        stack[stack[instructionPointer + 3]] = 1
    else:
        stack[stack[instructionPointer + 3]] = 0

    instructionPointer += 4
    return stack, instructionPointer


def eql(stack, instructionPointer, parameters, inp):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    if accumulator[0] == accumulator[1]:
        stack[stack[instructionPointer + 3]] = 1
    else:
        stack[stack[instructionPointer + 3]] = 0

    instructionPointer += 4
    return stack, instructionPointer


def main():
    lines = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
    lines = list(map(int, lines.split(",")))
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

    perm = list(itertools.permutations([0, 1, 2, 3, 4]))
    biggest = 0

    for x in range(len(perm)):
        inputs = [0, 0]
        for y in range(len(perm[x])):
            instructionPointer = 0
            stack = lines.copy()
            inputs[0] = perm[x][y]
            currentInput = 0
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
                if stack[instructionPointer] == 3:
                    setCurrentInput = True
                stack, instructionPointer = opcodes.get(op)(stack, instructionPointer, parameters, inputs[currentInput])
                
                if setCurrentInput:
                    currentInput = 1
                    setCurrentInput = False

            inputs[1] = stack[0]
        if(stack[0] > biggest):
            biggest = stack[0]
        inputs = [0, 0]
    print(biggest)




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

