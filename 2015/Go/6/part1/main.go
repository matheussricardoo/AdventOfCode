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
					grid[i][j] = 1
				} else if state == "OFF" {
					grid[i][j] = 0
				} else {
					if grid[i][j] == 1 {
						grid[i][j] = 0
					} else {
						grid[i][j] = 1
					}
				}

			}
		}
	}
	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			if grid[i][j] == 1 {
				total_lights++
			}
		}
	}
	fmt.Printf("Total lights: %d\n", total_lights)
}
