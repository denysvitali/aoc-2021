package main

import (
	"bytes"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"github.com/denysvitali/aoc-2021/go/day16/bitreader"
	"github.com/sirupsen/logrus"
	"github.com/stretchr/testify/assert"
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

func TestOperatorPacket3(t *testing.T){
	p := parsePacketFromString("8A004A801A8002F478")
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

func TestPartOneSample1(t *testing.T) {
	assert.Equal(t, 16, part1("input/sample1.txt"))
}

func TestPartOneSample2(t *testing.T) {
	assert.Equal(t, 12, part1("input/sample2.txt"))
}

func TestPartOneSample3(t *testing.T) {
	assert.Equal(t, 23, part1("input/sample3.txt"))
}

func TestPartOneSample4(t *testing.T) {
	assert.Equal(t, 31, part1("input/sample4.txt"))
}

func TestPartOneInput(t *testing.T) {
	assert.Equal(t, 879, part1("input/input.txt"))

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
