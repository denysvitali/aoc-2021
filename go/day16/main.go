package main

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"github.com/denysvitali/aoc-2021/go/day16/bitreader"
	"github.com/sirupsen/logrus"
	"os"
	"strings"
)

func readContent(path string) string {
	f, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}

	return string(f)
}

func parsePacketFromString(input string) Packet {
	bContent, err := hex.DecodeString(input)
	if err != nil {
		panic(err)
	}

	reader := bytes.NewReader(bContent)
	br := bitreader.New(reader)

	return parsePacket(br)
}

func parsePacket(br *bitreader.BitReader) Packet {
	if !br.HasBytes() {
		return nil
	}
	v := br.ReadBit(3)
	t := br.ReadBit(3)

	if t == 4 {
		p := LiteralPacket{}
		p.version = int(v)
		words := getWords(br)

		v := 0
		shift := 0
		for i := len(words) - 1; i>=0; i-- {
			v += int(words[i]) << shift
			shift += 4
		}

		p.value = uint64(v)
		return p
	}

	p := OperatorPacket{}
	p.t = int(t)
	p.version = int(v)

	// Get Length Type ID
	p.i = byte(br.ReadBit(1))

	switch p.i {
	case 0:
		// length is a 15-bit number representing the number of bits in the sub-packets
		bits := br.ReadBit(15)
		maxOffset := br.Offset() + int(bits)
		if bits == 0 {
			break
		}

		var subPackets []Packet
		for ;; {
			if br.Offset() == maxOffset {
				break
			}
			packet := parsePacket(br)
			if packet == nil {
				break
			}
			subPackets = append(subPackets, packet)
		}
		p.subPackets = subPackets
	case 1:
		nrPackets := br.ReadBit(11)
		var subPackets []Packet

		for ;; {
			if len(subPackets) == int(nrPackets) {
				break
			}

			packet := parsePacket(br)
			if packet == nil {
				break
			}
			subPackets = append(subPackets, packet)
		}

		p.subPackets = subPackets

	default:
		panic("???")
	}
	return p
}

func getWords(reader *bitreader.BitReader) []byte {
	var words []byte
	for ;; {
		start := reader.ReadBit(1)
		words = append(words, byte(reader.ReadBit(4)))
		if start == 0 {
			// Stop reading
			break
		}
	}
	return words
}

func getVersionSum(p Packet) int {
	acc := 0
	acc += p.Version()

	switch v:= p.(type) {
	case OperatorPacket:
		for _, s := range v.subPackets {
			acc += getVersionSum(s)
		}
	}

	return acc
}

func part1(path string) int {
	str := readContent(path)
	p := parsePacketFromString(str)
	return getVersionSum(p)
}

func part2(path string) int {
	str := readContent(path)
	p := parsePacketFromString(str)
	return p.Evaluate()
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
