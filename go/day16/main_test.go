package main

import (
	"bytes"
	"encoding/binary"
	"encoding/hex"
	"fmt"
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
	p := parsePacket("D2FE28")
	fmt.Printf("p=%+v\n", p)
}

func TestOperatorPacket1(t *testing.T){
	p := parsePacket("38006F45291200")
	fmt.Printf("p=%+v\n", p)
}

func TestOperatorParsePackets(t *testing.T){
	p := parsePacket("38006F45291200")
	switch v := p.(type) {
	case OperatorPacket:
		for _, p := range v.subPackets {
			parsePacket(hex.EncodeToString([]byte{byte(p)}))
		}
		fmt.Printf("subpackets=%+v", v.subPackets)
	}
	fmt.Printf("p=%+v\n", p)
}

func TestGetWord(t *testing.T){
	r := bytes.NewReader([]byte{0xD2, 0xFE, 0x28})
	b, _ := r.ReadByte()
	words := getWords(r, b)
	fmt.Printf("words=%v", words)
	expected := []byte{0x07, 0x0E, 0x05}
	if !bytes.Equal(words, expected) {
		t.Fatalf(
			"invalid words found: %02X found but %02X expected",
			words,
			expected,
		)
	}
}

func TestGetByte(t *testing.T){
	assert.Equal(t, byte(0x0F), getByte(0xFF,0, 4))
	assert.Equal(t, byte(0x0E), getByte(0xE0,0, 4))
	assert.Equal(t, byte(0x03), getByte(0b0011_1000,4, 5))
	assert.Equal(t, byte(0x07), getByte(0b0011_1000,3, 5))
}

func TestOperatorPacket2(t *testing.T){
	p := parsePacket("EE00D40C823060")
	fmt.Printf("p=%+v\n", p)
}

func TestSubPacket1(t *testing.T){
	p := parsePacket("2810") // 01010000001
	fmt.Printf("p=%+v\n", p)
}

func TestSubPacket2(t *testing.T){
	input := make([]byte, 2)
	binary.LittleEndian.PutUint16(input, 0b01010000001)
	h := hex.EncodeToString(input)
	logrus.Debugf("h=%s", h)
	p := parsePacket(h) // 01010000001
	fmt.Printf("p=%+v\n", p)
}

func TestPartOneSample(t *testing.T) {
	if part1("input/sample.txt") != 16 {
		t.Fatalf("invalid result")
	}
}

func TestWord2Bytes(t *testing.T) {
	a := byte(0b0111)
	b := byte(0b1110)
	c := byte(0b0101)

	output := words2bytes([]byte{a,b,c})
	if !bytes.Equal(output, []byte{0x07, 0xE5}) {
		t.Fatalf("invalid output: %02X", output)
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
