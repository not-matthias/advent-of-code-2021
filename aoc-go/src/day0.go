package main

import (
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

// Reads the input and returns it.
func readFile(fileName string) string {
	var content, err = os.ReadFile(fileName)
	check(err)

	return string(content)
}

func convertLines(content string) []int {
	var convertedLines []int
	for _, s := range strings.Split(content, "\n") {
		if converted, err := strconv.Atoi(s); err == nil {
			convertedLines = append(convertedLines, converted)
		}
	}

	return convertedLines
}

func solvePart1() {
	content := readFile("../aoc-rs/input/2021/day1.txt")
	lines := convertLines(content)

	result := 0
	for i := 1; i < len(lines); i++ {
		//fmt.Printf("%d < %d\n", lines[i-1], lines[i])
		if lines[i-1] < lines[i] {
			result += 1
		}
	}
	println(result)
}

func solvePart2() {
	content := readFile("../aoc-rs/input/2021/day1.txt")
	lines := convertLines(content)

	result := 0
	for i := 0; i < len(lines)-3; i++ {
		sum1 := lines[i] + lines[i+1] + lines[i+2]
		sum2 := lines[i+1] + lines[i+2] + lines[i+3]

		if sum1 < sum2 {
			result += 1
		}
	}
	println(result)
}

func main() {
	solvePart1()
	solvePart2()
}
