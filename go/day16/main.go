package main

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"github.com/sirupsen/logrus"
	"io"
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

type BitsPacket struct {

}

func parsePacket(input string) BitsPacket {
	bContent, err := hex.DecodeString(input)
	if err != nil {
		panic(err)
	}

	reader := bytes.NewReader(bContent)

	b, err := reader.ReadByte()
	if err != nil {
		panic(err)
	}
	// Packet Type
	v := (b & 0b11100000) >> 5
	t := (b & 0b00011100) >> 2

	logrus.Debugf("version=%d, type=%d", v, t)

	if t == 4 {
		// Literal value
		offset := 2
		for ;; {
			needsBits := 5 - offset
			theByte := byte(0)
			isEof := false
			logrus.Debugf("byte=%02X, needsBits=%d", b, needsBits)
			if needsBits > 0 {
				b2, err := reader.ReadByte()
				if err != nil {
					panic(err)
				}

				mask := byte(0xff) >> (8 - offset)
				theByte = b & mask << (5 - offset)

				mask = byte(0xff) << (8 - needsBits)
				second := (b2 & mask) >> (8 - needsBits)
				theByte += second
				offset = 8 - needsBits
				b = b2
			} else {
				mask := byte(0x1F)
				theByte = b & mask
				b2, err := reader.ReadByte()
				if err == io.EOF {
					isEof = true
				} else {
					b = b2
				}
			}

			logrus.Debugf("the_byte=%05b", theByte)
			if isEof {
				break
			}
		}
	} else {
		// Operator Packet
		// Get Length Type ID
		i := (b & 0b0000010) >> 1
		logrus.Debugf("length_type_id=%d", i)

		if i == 0 {
			// length is a 15-bit number representing the number of bits in the sub-packets
			l1 := b & 0x01
			b, err = reader.ReadByte()
			if err != nil {
				panic(err)
			}
			l2 := (b & 0xfe >> 1)
			logrus.Debugf("l_1 = %b, l_2 = %b", l1, l2)
		}

	}


	logrus.Infof("bytes=%v", b)
	return BitsPacket{}
}

func part1(path string) int {
	str := readContent(path)
	parsePacket(str)
	return -1
}

func part2(path string) int {
	return -1
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
