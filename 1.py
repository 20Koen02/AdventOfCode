import math

def calcFuel(mass):
    totalFuel = 0
    fuelMass = mass
    while True:
        fuelMass = math.trunc(fuelMass / 3) - 2
        if fuelMass <= 0:
            return totalFuel
        totalFuel += fuelMass
        

def readFile(file):
    with open(file) as f:
        return f.read().splitlines()

def main():
    lines = readFile("1_input.txt")
    totalFuel = 0
    for x in range(len(lines)):
        lines[x] = int(lines[x])
        totalFuel += calcFuel(lines[x])
    print(totalFuel)

if __name__ == "__main__":
    main()