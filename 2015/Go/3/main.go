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

	sx, sy := 0, 0
	rx, ry := 0, 0

	houses[Point{0, 0}] = 2

	for scanner.Scan() {
		line := scanner.Text()
		for i, char := range line {
			switch char {
			case '^':
				if i%2 == 0 {
					sy++
					houses[Point{sx, sy}]++
				} else if i%2 != 0 {
					ry++
					houses[Point{rx, ry}]++
				}
			case 'v':
				if i%2 == 0 {
					sy--
					houses[Point{sx, sy}]++
				} else if i%2 != 0 {
					ry--
					houses[Point{rx, ry}]++
				}
			case '>':
				if i%2 == 0 {
					sx++
					houses[Point{sx, sy}]++
				} else if i%2 != 0 {
					rx++
					houses[Point{rx, ry}]++
				}
			case '<':
				if i%2 == 0 {
					sx--
					houses[Point{sx, sy}]++
				} else if i%2 != 0 {
					rx--
					houses[Point{rx, ry}]++
				}
			default:
				panic("Character not assept.")
			}
		}
		fmt.Printf("How many houses receive at least one present? (%d)\n", len(houses))
	}
}
