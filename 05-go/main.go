package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"regexp"
	"strconv"
	"strings"
)

const gridWidth = 1000
const gridHeight = 1000

func readFile() []string {
	data, _ := ioutil.ReadFile("in.txt")
	return strings.Split(string(data), "\n")
}

func signum(x int) int {
	if x < 0 {
		return -1
	} else if x > 0 {
		return 1
	}
	return 0
}

func calculateOverlap(lines []string, allowDiagonals bool) int {
	re := regexp.MustCompile(",| -> ")
	grid := make([]int, gridWidth*gridHeight)
	gridCount := 0

	for _, line := range lines {
		coordinates := re.Split(line, 4)
		x1, _ := strconv.Atoi(coordinates[0])
		y1, _ := strconv.Atoi(coordinates[1])
		x2, _ := strconv.Atoi(coordinates[2])
		y2, _ := strconv.Atoi(coordinates[3])
		deltaX := x2 - x1
		deltaY := y2 - y1

		if !allowDiagonals && deltaX != 0 && deltaY != 0 {
			continue // Not a line (or diagonal)
		}

		// Length of the line
		var lineLength int
		if deltaX == 0 {
			lineLength = int(math.Abs(float64(deltaY)))
		} else {
			lineLength = int(math.Abs(float64(deltaX)))
		}

		// Direction of the line
		deltaX = signum(deltaX)
		deltaY = signum(deltaY)

		for i := 0; i <= lineLength; i++ {
			// Calculate the coordinates of the current point
			v := &grid[(y1+deltaY*i)*gridWidth+(x1+deltaX*i)]
			*v++
			// Check overlap
			if *v == 2 {
				gridCount++
			}
		}
	}

	return gridCount
}

func main() {
	fmt.Printf("Day 5 part one: %d\n", calculateOverlap(readFile(), false))
	fmt.Printf("Day 5 part two: %d\n", calculateOverlap(readFile(), true))
}
