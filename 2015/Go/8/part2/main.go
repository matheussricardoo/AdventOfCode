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
	encodedCount := 0

	for scanner.Scan() {
		input := scanner.Text()
		codeCount += len(input)
		encodedString := strconv.Quote(input)
		encodedCount += len(encodedString)
	}
	totalDiff := encodedCount - codeCount
	fmt.Printf("Total difference: %d\n", totalDiff)

}
