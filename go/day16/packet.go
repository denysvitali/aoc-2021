package main

type PacketType = string

const (
	Operator PacketType = "operator"
	Literal  PacketType = "literal"
)

type Packet interface {
	Type() PacketType
	Version() int
	Evaluate() int
}
