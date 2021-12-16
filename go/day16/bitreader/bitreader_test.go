package bitreader

import (
	"bytes"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestBitReader_ReadBit(t *testing.T) {
	input := []byte{0b0100_0010, 0xF0}
	br := New(bytes.NewReader(input))

	assert.Equal(t, uint64(0), br.ReadBit(1))
	assert.Equal(t, uint64(1), br.ReadBit(1))
	assert.Equal(t, uint64(0), br.ReadBit(1))
	assert.Equal(t, uint64(0), br.ReadBit(1))

	assert.Equal(t, uint64(0), br.ReadBit(1))
	assert.Equal(t, uint64(0), br.ReadBit(1))
	assert.Equal(t, uint64(1), br.ReadBit(1))
	assert.Equal(t, uint64(0), br.ReadBit(1))

	// Next Byte
	assert.Equal(t, uint64(1), br.ReadBit(1))
	assert.Equal(t, uint64(1), br.ReadBit(1))
	assert.Equal(t, uint64(1), br.ReadBit(1))
	assert.Equal(t, uint64(1), br.ReadBit(1))

	assert.Equal(t, uint64(0), br.ReadBit(1))
	assert.Equal(t, uint64(0), br.ReadBit(1))
	assert.Equal(t, uint64(0), br.ReadBit(1))
	assert.Equal(t, uint64(0), br.ReadBit(1))
}

func TestReadMultipleBits(t *testing.T) {
	input := []byte{0b0100_0010, 0xF0}
	br := New(bytes.NewReader(input))

	assert.Equal(t, uint64(4), br.ReadBit(4))
	assert.Equal(t, uint64(2), br.ReadBit(4))
}
