# sum operation
def sum(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)

    # add values
    out = accumulator[0] + accumulator[1]
    # set value
    stack[stack[instructionPointer + 3]] = out

    # set pointer and return new data
    instructionPointer += 4
    return stack, instructionPointer

# multiply operation
def mul(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)

    # multiply values
    out = accumulator[0] * accumulator[1]
    # set value
    stack[stack[instructionPointer + 3]] = out

    # set pointer and return new data
    instructionPointer += 4
    return stack, instructionPointer

# input operation
def inp(stack, instructionPointer, parameters):
    # get user input
    out = int(input("Please provide a single integer: "))
    # set value
    stack[stack[instructionPointer + 1]] = out

    # set pointer and return new data
    instructionPointer += 2
    return stack, instructionPointer

# output operation
def out(stack, instructionPointer, parameters):
    # print value
    if parameters[0] == 1:
        print(stack[instructionPointer + 1])
    else:
        print(stack[stack[instructionPointer + 1]])

    # set pointer and return new data
    instructionPointer += 2
    return stack, instructionPointer

# jump if true operation
def jit(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    # jump if value is not 0
    if accumulator[0] != 0:
        instructionPointer = accumulator[1]
    else:
        instructionPointer += 3

    # return new data
    return stack, instructionPointer

# jump if false operation
def jif(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    # jump if value is 0
    if accumulator[0] == 0:
        instructionPointer = accumulator[1]
    else:
        instructionPointer += 3

    # return new data
    return stack, instructionPointer

# less than operation
def lss(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    # set value if less than
    if accumulator[0] < accumulator[1]:
        stack[stack[instructionPointer + 3]] = 1
    else:
        stack[stack[instructionPointer + 3]] = 0

    # set pointer and return new data
    instructionPointer += 4
    return stack, instructionPointer

# equals operation
def eql(stack, instructionPointer, parameters):
    accumulator = getAccumulator(stack, instructionPointer, parameters)
    
    # set value if equals
    if accumulator[0] == accumulator[1]:
        stack[stack[instructionPointer + 3]] = 1
    else:
        stack[stack[instructionPointer + 3]] = 0

    # set pointer and return new data
    instructionPointer += 4
    return stack, instructionPointer

# machine
def main():
    # create stack from program
    stack = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
    stack = list(map(int, stack.split(",")))

    # init pointer
    instructionPointer = 0

    # assign operations to opcodes
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
        # check for exit code
        if stack[instructionPointer] == 99:
            break

        # set operation and parameters from current instruction
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

        # do operation and return new stack and pointer
        stack, instructionPointer = opcodes.get(op)(stack, instructionPointer, parameters)

# get values using the right mode (position or immediate mode)
def getAccumulator(stack, instructionPointer, parameters):
    accumulator = []
    for x in range(len(parameters)):
        if parameters[x] == 0:
            accumulator.append(stack[stack[instructionPointer + x + 1]])
        elif parameters[x] == 1:
            accumulator.append(stack[instructionPointer + x + 1])
    return accumulator

# init main
if __name__ == "__main__":
    main()
