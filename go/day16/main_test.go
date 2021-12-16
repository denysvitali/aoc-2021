package main

import (
	"fmt"
	"github.com/sirupsen/logrus"
	"testing"
)

func TestLiteralValue(t *testing.T) {
	logrus.SetLevel(logrus.DebugLevel)
	p := parsePacket("D2FE28")
	fmt.Printf("p=%v\n", p)
}

func TestPartOneSample(t *testing.T) {
	if part1("input/sample.txt") != 16 {
		t.Fatalf("invalid result")
	}
}

func TestPartOneInput(t *testing.T) {
	res := part1("input/input.txt")
	expected := 1644
	if res != expected {
		t.Fatalf("got %d but %d expected", res, expected)
	}
}

func TestPartTwoSample(t *testing.T) {
	res := part2("input/sample.txt")
	expected := 195
	if res != expected {
		t.Fatalf("got %d but %d expected", res, expected)
	}
}

func TestPartTwoInput(t *testing.T) {
	res := part2("input/input.txt")
	expected := 229
	if res != expected {
		t.Fatalf("got %d but %d expected", res, expected)
	}
}
