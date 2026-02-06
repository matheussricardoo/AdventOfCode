package main

import (
	"crypto/md5"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := "iwrupvqb"
	count := 0
	for {
		sum_input := input + strconv.Itoa(count)
		sum_bytes := []byte(sum_input)
		hashString := md5.Sum(sum_bytes)
		convertHash := fmt.Sprintf("%x", hashString)
		if strings.HasPrefix(convertHash, "000000") {
			fmt.Printf("The number is: %d\n", count)
			break
		} else {
			count++
		}
	}
}
