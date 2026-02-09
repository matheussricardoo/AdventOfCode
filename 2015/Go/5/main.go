package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	good_string := 0

	for scanner.Scan() {
		input := scanner.Text()
		hasSandwich := false
		letter_repeat := false

		for i := 0; i < len(input)-2; i++ {
			if input[i] == input[i+2] {
				hasSandwich = true
			}
			for j := i + 2; j < len(input)-1; j++ {
				if input[i] == input[j] && input[i+1] == input[j+1] {
					letter_repeat = true
				}
			}
		}
		if letter_repeat && hasSandwich {
			good_string++
		}
	}
	fmt.Printf("%d\n", good_string)
}
