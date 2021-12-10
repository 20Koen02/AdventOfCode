import numpy as np
from PIL import Image

def getInput():
    with open("inout/8_input.txt") as f:
        lines = f.read()

    return lines.strip()

def main():
    lines = getInput()
    w = 25
    h = 6

    pixels = w * h
    layers = len(lines) // pixels
    
    data = np.zeros((h, w, 3), dtype=np.uint8)

    for i in range(pixels):
        for j in range(layers):
            if lines[(j*pixels)+i] != "2":
                print("■ " if lines[(j*pixels)+i] == "1" else "□ ", end="")
                break
        if (i+1) % w == 0:
            print()
    



if __name__ == "__main__":
    main()