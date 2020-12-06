package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func stringToInt(str string) (int, error) {
	nonFractionalPart := strings.Split(str, ".")
	return strconv.Atoi(nonFractionalPart[0])
}

// readLines reads a whole file into memory
// and returns a slice of its lines.
func readLines(path string) ([]point, rectangle, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, rectangle{}, err
	}
	defer file.Close()

	var lines []point
	mida := rectangle{}
	esPrimer := true
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {

		punt := strings.Split(scanner.Text(), ", ")
		novaX, _ := stringToInt(punt[0])
		novaY, _ := stringToInt(punt[1])
		if esPrimer {
			mida = rectangle{novaX, novaX, novaY, novaY}
			esPrimer = false
		} else {
			mida.comprovaMida(novaX, novaY)
		}

		lines = append(lines, point{novaX, novaY})
	}
	return lines, mida, scanner.Err()
}

type point struct {
	x int
	y int
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func (p point) distance(x int, y int) int {
	return abs(x-p.x) + abs(y-p.y)
}

type rectangle struct {
	minX int
	maxX int
	minY int
	maxY int
}

func (s *rectangle) comprovaMida(x int, y int) {
	if x < s.minX {
		s.minX = x
	}
	if x > s.maxX {
		s.maxX = x
	}
	if y < s.minY {
		s.minY = y
	}
	if y > s.maxY {
		s.maxY = y
	}
}

func main() {
	max := 10000
	points, area, err := readLines("input")
	if err != nil {
		panic("File read failed")
	}
	correctes2 := mesPropers(points, area, max)

	fmt.Println("Part 2: ", correctes2)
}

type distances struct {
	punt      point
	distances []int
}

func mesPropers(punts []point, area rectangle, limit int) int {
	max := 0
	for actualy := 0; actualy <= area.maxY+1; actualy++ {
		for actualx := 0; actualx <= area.maxX+1; actualx++ {
			distances := 0
			for _, punt := range punts {
				distances += punt.distance(actualx, actualy)
			}
			if distances < limit {
				max = max + 1
			}
		}
	}

	return max
}
