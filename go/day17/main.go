package main

import (
	"fmt"
	"github.com/sirupsen/logrus"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Coord struct {
	x int
	y int
}

/*
	Motion represents the trajectory
*/
type Motion struct {
	originalV Coord
	firstStep bool
	v         Coord
	pos       Coord
	highest   Coord
}

func (m *Motion) step() Coord {
	if m.firstStep {
		m.firstStep = false
		m.originalV = m.v
	}

	x := m.pos.x + m.v.x
	y := m.pos.y + m.v.y

	// Decrease speed in X
	if m.v.x > 0 {
		m.v.x--
	} else if m.v.x < 0 {
		m.v.x++
	}

	// Decrease speed in Y
	m.v.y--

	m.pos = Coord{x: x, y: y}

	if m.pos.y > m.highest.y {
		m.highest = m.pos
	}

	return m.pos
}

// target area: x=20..30, y=-10..-5
var inputRe = regexp.MustCompile("^target area: x=([-]?\\d+)\\.\\.([-]?\\d+), y=([-]?\\d+)\\.\\.([-]?\\d+)$")

func parseFile(path string) (start Coord, end Coord) {
	content, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	m := inputRe.FindStringSubmatch(string(content))
	if len(m) == 0 {
		panic("input doesn't match regex")
	}

	x1, _ := strconv.ParseInt(m[1], 10, 32)
	x2, _ := strconv.ParseInt(m[2], 10, 32)
	y1, _ := strconv.ParseInt(m[3], 10, 32)
	y2, _ := strconv.ParseInt(m[4], 10, 32)

	start.x = int(math.Min(float64(x1), float64(x2)))
	start.y = int(math.Max(float64(y1), float64(y2)))

	end.x = int(math.Max(float64(x1), float64(x2)))
	end.y = int(math.Min(float64(y1), float64(y2)))

	return start, end
}

func NewMotion(vx int, vy int) Motion {
	return Motion{
		originalV: Coord{x: vx, y: vy},
		firstStep: true,
		v:         Coord{x: vx, y: vy},
		pos:       Coord{x: 0, y: 0},
	}
}

func getTrajectories(start Coord, end Coord) (targetReached int, best Motion) {
	// Choose aim
	best = Motion{highest: Coord{x: 0, y: 0}}
	targetReached = 0

	// There might be a smarter way to do this
	for i:=-end.x; i<=end.x; i++ {
		for j := -200; j <= 200; j++ {
			m := NewMotion(i, j)
			for {
				m.step()
				if m.pos.x > end.x {
					break
				}

				if m.pos.y < end.y {
					break
				}

				if m.pos.x >= start.x &&
					m.pos.y <= start.y &&
					m.pos.x <= end.x &&
					m.pos.y >= end.y {
					targetReached++
					if m.highest.y > best.highest.y {
						best = m
					}
					break
				}
			}
		}
	}
	return targetReached, best
}

func part1(path string) int {
	start, end := parseFile(path)
	_, b := getTrajectories(start, end)
	return b.highest.y
}

func part2(path string) int {
	start, end := parseFile(path)
	t, _ := getTrajectories(start, end)
	return t
}


func main() {
	level := os.Getenv("LOG_LEVEL")
	if strings.ToLower(level) == "debug" {
		logrus.SetLevel(logrus.DebugLevel)
	}
	if len(os.Args) == 1 {
		logrus.Fatalf("Usage: %s input|sample", os.Args[0])
	}

	fileInput := os.Args[1]
	switch fileInput {
	case "input":
		fileInput = "input/input.txt"
	case "sample":
		fileInput = "input/sample.txt"
	default:
		logrus.Fatalf("invalid argument %s", fileInput)
	}

	fmt.Printf("Part 1: %v\n", part1(fileInput))
	fmt.Printf("Part 2: %v\n", part2(fileInput))
}
