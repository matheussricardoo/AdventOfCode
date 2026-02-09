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
		vowels := 0
		hasDouble := false
		hasForbidden := false
		var lastChar rune

		for _, char := range input {
			if char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' {
				vowels++

			}
			if char == lastChar {
				hasDouble = true
			}
			if lastChar == 'a' && char == 'b' || lastChar == 'c' && char == 'd' || lastChar == 'p' && char == 'q' || lastChar == 'x' && char == 'y' {
				hasForbidden = true
			}

			lastChar = char
		}
		if vowels >= 3 && hasDouble == true && hasForbidden == false {
			good_string++
		}
	}
	fmt.Printf("%d\n", good_string)
}
