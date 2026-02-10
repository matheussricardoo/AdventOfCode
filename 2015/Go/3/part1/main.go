package main

import (
	"bufio"
	"fmt"
	"os"
)

type Point struct {
	X int
	Y int
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	houses := make(map[Point]int)

	var point Point

	x := point.X
	y := point.Y

	houses[Point{0, 0}] = 1

	for scanner.Scan() {
		line := scanner.Text()
		for _, char := range line {
			switch char {
			case '^':
				y++
			case 'v':
				y--
			case '>':
				x++
			case '<':
				x--
			default:
				panic("Character not assept.")
			}
			present := Point{x, y}
			houses[present]++
		}
		fmt.Printf("How many houses receive at least one present? (%d)\n", len(houses))
	}
}
