package main

import (
	"bytes"
	"encoding/binary"
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

func getLeftMask(size int) byte {
	return 0xFF << (8 - size)
}

func getRightMask(size int) byte {
	return 0xFF >> (8 - size)
}

func parsePacket(input string) Packet {
	bContent, err := hex.DecodeString(input)
	if err != nil {
		panic(err)
	}

	reader := bytes.NewReader(bContent)

	b, err := reader.ReadByte()
	if err != nil {
		panic(err)
	}

	v := (b & 0b11100000) >> 5
	t := (b & 0b00011100) >> 2
	logrus.Debugf("version=%d, type=%d", v, t)


	if t == 4 {
		p := LiteralPacket{}
		words := getWords(reader, b)
		w2b := words2bytes(words)

		switch len(w2b) {
		case 1:
			p.value = uint64(binary.BigEndian.Uint16([]byte{0x00, w2b[0]}))
		}
		if len(w2b) == 2 {
			p.value = uint64(binary.BigEndian.Uint16(w2b))
		}

		return p
	}

	logrus.Debugf("This is an operator packet")

	p := OperatorPacket{}

	// Get Length Type ID
	p.i = (b & 0b0000010) >> 1
	logrus.Debugf("length_type_id=%d", p.i)

	switch p.i {
	case 0:
		// length is a 15-bit number representing the number of bits in the sub-packets
		l1 := b & getRightMask(1)
		b, err = reader.ReadByte()
		l2 := b & getLeftMask(8)
		b, err = reader.ReadByte()
		l3 := b & getLeftMask(6) >> (8 - 6)
		p.l = binary.BigEndian.Uint32([]byte{0x00, l1, l2, l3})

		// SubPackets
		var subPackets []byte
		initialByte := b & getRightMask(2)

		subPackets = append(subPackets, initialByte)
		currentLength := 0
		for ;; {
			b, err := reader.ReadByte()
			if err == io.EOF {
				break
			}

			if currentLength + 8 < int(p.l) {
				subPackets = append(subPackets, b)
			} else {
				subPackets = append(subPackets, getByte(b, 0, int(p.l) - currentLength))
			}
		}
		p.subPackets = subPackets

	case 1:
		l1 := b & getRightMask(1)
		b, err = reader.ReadByte()
		l2 := b & getLeftMask(8)
		b, err = reader.ReadByte()
		l3 := b & getLeftMask(2) >> (8 - 2)
		p.l = binary.BigEndian.Uint32([]byte{0x00, l1, l2, l3})
	default:
		panic("???")
	}
	return p
}

/*
	words2bytes converts an array of 4-bit words
	into an array of 8-bit (1-byte) words.
*/
func words2bytes(words []byte) []byte {
	if len(words)%2 != 0 {
		words = append([]byte{0x00}, words...)
	}

	var prev = -1
	var output []byte
	for _, w := range words {
		if prev == -1 {
			prev = int(w)
			continue
		}

		output = append(output, (byte(prev)<<4)+w)
		prev = -1
	}

	return output
}

func getByte(b byte, from int, to int) byte {
	m := byte(0xFF)
	if from > 0 {
		m >>= from - 1
	}

	if to > 0 {
		m &= 0xFF << (8 - to)
	}
	b = (b & m) >> (8 - to)
	return b
}

func getWords(reader *bytes.Reader, b byte) []byte {
	// XXXXXX10
	offset := 6
	wordSize := 5
	var words []byte
	for {
		if offset + wordSize > 8 {
			// Need another byte
			b2, err := reader.ReadByte()
			if err == io.EOF {
				break
			}

			first := getByte(b, offset, 8)
			second := getByte(b2, 0, wordSize - (8 - offset))

			first <<= wordSize - (8 - offset)
			l := first + second

			b = b2
			words = append(words, l)
			offset += wordSize
		} else {
			word := getByte(b, offset, offset + wordSize)
			words = append(words, word)
			if offset+wordSize == 8 {
				var err error
				b, err = reader.ReadByte()
				if err == io.EOF {
					break
				}
			}
			offset += wordSize
		}
		offset %= 8
	}

	return words
}

/*
	get4BitWord returns, given a 5-bit word,
	a 4-bit word (stripping away the first bit)
*/
func get4BitWord(word byte) byte {
	return word & 0x0F
}

func isEndOfPacket(word byte) bool {
	return word&0x10 == 0
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
