package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Box struct {
	L, W, H int
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	totalPaper := 0

	for scanner.Scan() {
		input := scanner.Text()

		var box Box

		result := strings.Split(input, "x")

		box.L, _ = strconv.Atoi(result[0])
		box.W, _ = strconv.Atoi(result[1])
		box.H, _ = strconv.Atoi(result[2])

		list := []int{box.L, box.W, box.H}
		sort.Ints(list)

		area := 2*box.L*box.W + 2*box.W*box.H + 2*box.H*box.L

		slack := list[0] * list[1]

		totalPaper += area + slack

	}
	fmt.Printf("Result: %d\n", totalPaper)
}
