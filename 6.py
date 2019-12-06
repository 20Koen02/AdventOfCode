


def main():
    orbitMap = getInput()
    hops = 0
    currentObject = ""
    for x in range(len(orbitMap)):
        currentObject = list(orbitMap.values())[x]
        while True:
            if currentObject == None:
                break
            hops += 1
            currentObject = orbitMap.get(currentObject)
    print(hops)


def getInput():
    with open("inout/6_input.txt") as f:
        file = f.read()
    lines = file.splitlines()
    lines.reverse()
    linesDict = {}
    for x in range(len(lines)):
        lines[x] = lines[x].split(")")
        linesDict[lines[x][1]] = lines[x][0]
    print(linesDict)
    return linesDict

if __name__ == "__main__":
    main()