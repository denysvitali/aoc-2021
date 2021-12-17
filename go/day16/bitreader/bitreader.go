package bitreader

import (
	"bytes"
	"io"
	"math"
)

type BitReader struct {
	reader       *bytes.Reader
	lastByteRead byte
	offset       int
}

func (b *BitReader) ReadBit(amount int) uint64 {
	shift := 0
	cb := byte(0x00)
	for i := amount - 1; i>=0; i-- {
		if b.offset % 8 == 0 {
			var err error
			b.lastByteRead, err = b.reader.ReadByte()
			if err != nil {
				panic(err)
			}
		}
		relBytePos := b.offset % 8

		maskL := byte(math.Pow(2, float64(8 - (relBytePos)) - 1))
		maskR := byte(math.Pow(2, float64(8 - (relBytePos + 1))))

		mask := maskL & maskR
		masked := b.lastByteRead & mask

		bs := masked >> (8 - (b.offset % 8) - 1)
		fs := bs << i
		cb += fs
		b.offset++
		shift++
	}

	return uint64(cb)
}

func (b *BitReader) Offset() int {
	return b.offset
}

func (b *BitReader) HasBytes() bool {
	_, e := b.reader.ReadByte()
	if e == io.EOF {
		return false
	}
	_ = b.reader.UnreadByte()
	return true
}

func New(reader *bytes.Reader) *BitReader {
	br := BitReader{
		reader: reader,
		offset: 0,
	}
	return &br
}
