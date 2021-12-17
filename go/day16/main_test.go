package main

import (
	"bytes"
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

func TestOperatorSum(t *testing.T) {
	p := parsePacketFromString("C200B40A82")
	assert.Equal(t, 3, p.Evaluate())
}

func TestOperatorProduct(t *testing.T) {
	p := parsePacketFromString("04005AC33890")
	assert.Equal(t, 54, p.Evaluate())
}

func TestOperatorMin(t *testing.T) {
	p := parsePacketFromString("880086C3E88112")
	assert.Equal(t, 7, p.Evaluate())
}

func TestOperatorMax(t *testing.T) {
	p := parsePacketFromString("CE00C43D881120")
	assert.Equal(t, 9, p.Evaluate())
}

func TestOperatorLt(t *testing.T) {
	p := parsePacketFromString("D8005AC2A8F0")
	assert.Equal(t, 1, p.Evaluate())
}

func TestOperatorComplete(t *testing.T) {
	p := parsePacketFromString("9C0141080250320F1802104A08")
	assert.Equal(t, 1, p.Evaluate())
}

func TestPartOneInput(t *testing.T) {
	assert.Equal(t, 879, part1("input/input.txt"))
}

func TestPartTwoInput(t *testing.T) {
	assert.Equal(t, 539051801941, part2("input/input.txt"))

}
