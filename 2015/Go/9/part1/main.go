package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

var minDist = math.MaxInt64
var distances = map[string]map[string]int{}

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		input := scanner.Text()
		parts := strings.Fields(input)
		cityA := parts[0]
		cityB := parts[2]
		distanceNumber, _ := strconv.Atoi(parts[4])

		if distances[cityA] == nil {
			distances[cityA] = make(map[string]int)
		}
		distances[cityA][cityB] = distanceNumber

		if distances[cityB] == nil {
			distances[cityB] = make(map[string]int)
		}
		distances[cityB][cityA] = distanceNumber
	}
	totalCities := len(distances)

	for startCity := range distances {
		visited := map[string]bool{}
		visited[startCity] = true

		citiesVisited(startCity, visited, 0, totalCities)
	}
	fmt.Println(minDist)
}

func citiesVisited(city string, visited map[string]bool, totalDistance int, totalCities int) {
	if len(visited) == totalCities {
		if totalDistance < minDist {
			minDist = totalDistance
		}
		return
	}
	for neighbor, dist := range distances[city] {
		if !visited[neighbor] {
			visited[neighbor] = true
			citiesVisited(neighbor, visited, totalDistance+dist, totalCities)
			delete(visited, neighbor)
		}
	}

}
