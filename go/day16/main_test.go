package main

import (
	"bytes"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"github.com/denysvitali/aoc-2021/go/day16/bitreader"
	"github.com/sirupsen/logrus"
	"os"
	"testing"
)

func TestMain(m *testing.M){
	logrus.SetLevel(logrus.DebugLevel)
	os.Exit(m.Run())
}

func TestLiteralValue(t *testing.T) {
	p := parsePacketFromString("D2FE28")
	fmt.Printf("p=%+v\n", p)
}

func TestLiteralValue2(t *testing.T) {
	reader := bytes.NewReader([]byte{0x06, 0x8A})
	br := bitreader.New(reader)
	br.ReadBit(5)
	p := parsePacket(br)
	fmt.Printf("p=%+v\n", p)
}

func TestOperatorPacket1(t *testing.T){
	p := parsePacketFromString("38006F45291200")
	fmt.Printf("p=%+v\n", p)
}

func TestGetWord(t *testing.T){
	r := bytes.NewReader([]byte{0x02, 0xFE, 0x28})
	br := bitreader.New(r)
	br.ReadBit(6)
	words := getWords(br)
	fmt.Printf("words=%v\n", words)
	expected := []byte{0x07, 0x0E, 0x05}
	if !bytes.Equal(words, expected) {
		t.Fatalf(
			"invalid words found: %02X found but %02X expected",
			words,
			expected,
		)
	}
}

func TestOperatorPacket2(t *testing.T){
	p := parsePacketFromString("EE00D40C823060")
	fmt.Printf("p=%+v\n", p)
}

func TestSubPacket1(t *testing.T){
	p := parsePacketFromString("2810") // 01010000001
	fmt.Printf("p=%+v\n", p)
}

func TestSubPacket2(t *testing.T){
	input := make([]byte, 2)
	binary.LittleEndian.PutUint16(input, 0b01010000001)
	h := hex.EncodeToString(input)
	logrus.Debugf("h=%s", h)
	p := parsePacketFromString(h) // 01010000001
	fmt.Printf("p=%+v\n", p)
}

func TestPartOneSample(t *testing.T) {
	if part1("input/sample.txt") != 16 {
		t.Fatalf("invalid result")
	}
}

func TestParseNumber(t *testing.T){

}

func TestGetMask(t *testing.T) {
	mask := getLeftMask(5)
	if mask != 0b1111_1000 {
		t.Fatalf("invalid mask: %08b", mask)
	}

	mask = getRightMask(5)
	if mask != 0b0001_1111 {
		t.Fatalf("invalid mask: %08b", mask)
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
