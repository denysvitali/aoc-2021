package main

type LiteralPacket struct {
	value uint64
}

func (l LiteralPacket) Type() PacketType {
	return Literal
}

var _ Packet = (*LiteralPacket) (nil)