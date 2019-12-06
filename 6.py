


def main():
    orbitMap = getInput()

    bfs(orbitMap, "YOU", "SAN")

    # for x in range(len(orbitMap)):
    #     currentObject = list(orbitMap.values())[x]
    #     while True:
    #         if currentObject == None:
    #             break
    #         hops += 1
    #         currentObject = orbitMap.get(currentObject)
    # print(hops)


def getInput():
    with open("inout/6_input.txt") as f:
        file = f.read()
    lines = file.splitlines()
    lines.reverse()
    tree = {}
    

    for x in range(len(lines)):
        lines[x] = lines[x].split(")")
        
        
    for x in range(len(lines)):
        if tree.get(lines[x][0]) == None:
            tree[lines[x][0]] = [lines[x][1]]
        else:
            tree[lines[x][0]].append(lines[x][1])
    

    print(tree)
    return lines

if __name__ == "__main__":
    main()