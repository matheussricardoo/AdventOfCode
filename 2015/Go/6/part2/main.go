package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	state := ""
	var grid [1000][1000]int
	total_lights := 0
	x1, y1, x2, y2 := 0, 0, 0, 0
	for scanner.Scan() {
		input := scanner.Text()
		if strings.HasPrefix(input, "turn on") {
			state = "ON"
			fmt.Sscanf(input, "turn on %d,%d through %d,%d", &x1, &y1, &x2, &y2)
		} else if strings.HasPrefix(input, "turn off") {
			state = "OFF"
			fmt.Sscanf(input, "turn off %d,%d through %d,%d", &x1, &y1, &x2, &y2)
		} else {
			state = "REVERSE"
			fmt.Sscanf(input, "toggle %d,%d through %d,%d", &x1, &y1, &x2, &y2)
		}

		for i := x1; i <= x2; i++ {
			for j := y1; j <= y2; j++ {
				if state == "ON" {
					grid[i][j]++
				} else if state == "OFF" {
					if grid[i][j] > 0 {
						grid[i][j]--
					}
				} else {
					grid[i][j] += 2
				}
			}
		}
	}
	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			total_lights += grid[i][j]
		}
	}
	fmt.Printf("Total lights: %d\n", total_lights)
}
