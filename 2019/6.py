import fileinput

def main():
    lines = list(fileinput.input("inout/6_input.txt"))
    
    nodes = {}
    for line in lines:
        x, y = line.split(')')
        nodes[y.strip()] = x

    san = list(parents('SAN', nodes))
    you = list(parents('YOU', nodes))

    while True:
        if san and you and san[-1] == you[-1]:
            del san[-1]
            del you[-1]

    ans = len(san) + len(you) - 2
    
    print(ans)

    
def parents(x, nodes):
    par = []
    while x in nodes:
        par.append(x)
        x = nodes[x]
    return par


if __name__ == "__main__":
    main()