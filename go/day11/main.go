package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"

	"github.com/sirupsen/logrus"
)

func parse_input(str string) Matrix {
	o := [][]uint8{}
	for _, row := range strings.Split(str, "\n") {
		r := []uint8{}
		for _, c := range row {
			v, err := strconv.ParseInt(string(c), 10, 8)
			if err != nil {
				panic(err)
			}
			r = append(r, uint8(v))
		}
		o = append(o, r)
	}

	return Matrix{
		content:         o,
		already_flashed: map[Coord]bool{},
		flashes:         0,
	}
}

type Coord struct {
	x uint8
	y uint8
}

type Matrix struct {
	content         [][]uint8
	already_flashed map[Coord]bool
	flashes         uint
}

func (m *Matrix) print() {
	for _, r := range m.content {
		for _, v := range r {
			fmt.Printf("%d", v)
		}
		fmt.Println()
	}
	fmt.Println()
}

func (m *Matrix) adj(c Coord) []Coord {
	max_x := int(len(m.content[c.y]))
	max_y := int(len(m.content))

	coords := []Coord{}

	for x := int(c.x) - 1; x <= int(c.x)+1; x++ {
		for y := int(c.y) - 1; y <= int(c.y)+1; y++ {
			if x == int(c.x) && y == int(c.y) {
				continue
			}

			if x < 0 || y < 0 || x >= max_x || y >= max_y {
				continue
			}
			coords = append(coords,
				Coord{x: uint8(x), y: uint8(y)},
			)
		}
	}
	return coords
}

func (m *Matrix) flash(coord Coord) {
	if v, ok := m.already_flashed[coord]; ok && v {
		return
	}

	m.already_flashed[coord] = true
	m.flashes++

	adj := m.adj(coord)
	for _, c := range adj {
		m.content[c.y][c.x] += 1
	}

	for _, c := range adj {
		if m.content[c.y][c.x] > 9 {
			// Flash
			m.flash(c)
		}
	}

}

func (m *Matrix) step() {
	// Increase by 1
	for y, r := range m.content {
		for x, _ := range r {
			m.content[y][x] += 1
		}
	}

	for y, r := range m.content {
		for x, e := range r {
			if e > 9 {
				m.flash(Coord{
					x: uint8(x),
					y: uint8(y),
				})
			}
		}
	}

	for c := range m.already_flashed {
		m.content[c.y][c.x] = 0
	}

	m.already_flashed = map[Coord]bool{}
}

func read_content(path string) string {
	f, err := os.Open(path)
	if err != nil {
		panic(fmt.Errorf("unable to open file: %v", err))
	}

	content, err := ioutil.ReadAll(f)
	if err != nil {
		panic(fmt.Errorf("unable to read file: %v", err))
	}

	input := string(content)
	return input
}

func part1(path string) int {
	str := read_content(path)
	m := parse_input(str)
	for i := 1; i <= 100; i++ {
		m.step()
	}
	return int(m.flashes)
}

func part2(path string) int {
	str := read_content(path)
	m := parse_input(str)
	last_flashes := uint(0)

	step := 0
	for {
		step++
		m.step()
		diff := m.flashes - last_flashes
		fmt.Printf("%d\n", diff)
		if diff == 10*10 {
			return step
		}
		last_flashes = m.flashes
	}
}

func main() {
	l := logrus.New()
	if len(os.Args) == 1 {
		l.Fatalf("Usage: %s input|sample", os.Args[0])
	}

	fileInput := os.Args[1]
	switch fileInput {
	case "input":
		fileInput = "input/input.txt"
	case "sample":
		fileInput = "input/sample.txt"
	default:
		l.Fatalf("invalid argument %s", fileInput)
	}

	fmt.Printf("Part 1: %v\n", part1(fileInput))
	fmt.Printf("Part 2: %v\n", part2(fileInput))
}
