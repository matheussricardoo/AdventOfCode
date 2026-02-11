package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	dic := map[string]string{}
	memo := map[string]int{}

	for scanner.Scan() {
		input := scanner.Text()
		pieces := strings.Split(input, " -> ")
		dic[pieces[1]] = pieces[0]

	}

	result := getValue("a", memo, dic)
	fmt.Printf("Signal on wire a: %d\n", result)
}

func getValue(wire string, memo map[string]int, dic map[string]string) int {
	val, err := strconv.Atoi(wire)

	if err == nil {
		return val
	}

	if savedValue, ok := memo[wire]; ok {
		return savedValue
	}

	formulate := dic[wire]
	parts := strings.Fields(formulate)
	var result int

	if len(parts) == 1 {
		result = getValue(parts[0], memo, dic)
	} else if len(parts) == 2 {
		val := getValue(parts[1], memo, dic)
		result = int(^uint16(val))
	} else {

		left := getValue(parts[0], memo, dic)
		right := getValue(parts[2], memo, dic)

		switch parts[1] {
		case "AND":
			result = left & right
		case "OR":
			result = left | right
		case "LSHIFT":
			result = left << right
		case "RSHIFT":
			result = left >> right
		}
	}

	memo[wire] = result
	return result
}
