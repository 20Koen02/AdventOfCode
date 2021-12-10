
def getInput(file):
    with open(file) as f:
        first, second = f.read().split('\n')
    
    return [x.split(',') for x in [first, second]]


def getCoords(line):
    DisX = {'L': -1, 'R': 1, 'U': 0, 'D': 0}
    DisY = {'L': 0, 'R': 0, 'U': 1, 'D': -1}

    currLoc = [0, 0]
    length = 0
    answer = {}
    for location in line:
        operation = location[0]
        amount = int(location[1:])
        for x in range(amount):
            currLoc[0] += DisX[operation]
            currLoc[1] += DisY[operation]
            length += 1
            if (currLoc[0],currLoc[1]) not in answer:
                answer[(currLoc[0],currLoc[1])] = length
    return answer

def main():
    first, second = getInput("inout/3_input.txt")

    first = getCoords(first)
    second = getCoords(second)
    intersections = set(first.keys())&set(second.keys())

    part1 = min([abs(x)+abs(y) for (x,y) in intersections])
    part2 = min([first[p]+second[p] for p in intersections])
    print(part1, part2)


if __name__ == "__main__":
    main()