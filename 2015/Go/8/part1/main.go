package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	codeCount := 0
	memStringCount := 0

	for scanner.Scan() {
		input := scanner.Text()
		codeCount += len(input)
		memString, _ := strconv.Unquote(input)
		memStringCount += len(memString)
	}
	totalDiff := codeCount - memStringCount
	fmt.Printf("Total difference: %d\n", totalDiff)

}
