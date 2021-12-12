package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"reflect"
	"testing"
)

func Test1(t *testing.T) {
	m := parse_input("11111\n19991\n19191\n19991\n11111")

	fmt.Printf("Step 1\n")

	m.step()

	expectedM := parse_input("34543\n40004\n50005\n40004\n34543")

	if !reflect.DeepEqual(expectedM.content, m.content) {
		t.Fatalf("unexpected content")
	}
	m.print()

	fmt.Printf("Step 2\n")
	m.step()
	m.print()
}

func TestPartOneSampleStep(t *testing.T) {
	f, err := os.Open("input/sample.txt")
	if err != nil {
		t.Fatal(err)
	}

	content, err := ioutil.ReadAll(f)
	if err != nil {
		t.Fatal(err)
	}

	m := parse_input(string(content))
	expected2 := parse_input("8807476555\n5089087054\n8597889608\n8485769600\n8700908800\n6600088989\n6800005943\n0000007456\n9000000876\n8700006848")

	for i := 1; i <= 10; i++ {
		fmt.Printf("Step %d\n", i)
		m.step()
		m.print()

		if i == 2 {
			if !reflect.DeepEqual(
				m.content,
				expected2.content,
			) {
				t.Fatalf("not equal")
			}
		}
	}

	if m.flashes != 204 {
		t.Fatalf("unexpected number of flashes: %d", m.flashes)
	}
}

func TestPartOneSample(t *testing.T) {
	if part1("input/sample.txt") != 1656 {
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
