import collections

def sum(stack, instructionPointer):
    out = stack[stack[instructionPointer + 1]] + stack[stack[instructionPointer + 2]]
    stack[stack[instructionPointer + 3]] = out
    return stack

def multiply(stack, instructionPointer):
    out = stack[stack[instructionPointer + 1]] * stack[stack[instructionPointer + 2]]
    stack[stack[instructionPointer + 3]] = out
    return stack

def readInput(file):
    with open(file) as f:
        fileContents = f.read()
    codeList = fileContents.split(",")
    for x in range(len(codeList)):
        codeList[x] = int(codeList[x])
    return codeList

def writeOutput(file, stack):
    for x in range(len(stack)):
        stack[x] = str(stack[x])
    with open(file, "w+") as f:
        f.write(",".join(stack))
    exit()

def main():
    code = readInput("2_input.txt")
    ans = 19690720
    reached = False

    for x in range(0, 99):
        if reached == True:
            break
        for y in range(0, 99):
            instructionPointer = 0
            stack = code.copy()
            stack[1] = x
            stack[2] = y
            for z in range(len(stack)):
                if stack[instructionPointer] == 1:
                    stack = sum(stack, instructionPointer)
                    instructionPointer += 4
                elif stack[instructionPointer] == 2:
                    stack = multiply(stack, instructionPointer)
                    instructionPointer += 4
                elif stack[instructionPointer] == 99:
                    break
                else:
                    print("Unknown opcode")
                    break

            if stack[0] == ans:
                reached = True
                break


    writeOutput("2_output.txt", stack)

if __name__ == "__main__":
    main()