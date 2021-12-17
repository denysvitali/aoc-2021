package main

type LiteralPacket struct {
	value   uint64
	version int
}

func (l LiteralPacket) Evaluate() int {
	return int(l.value)
}

func (l LiteralPacket) Version() int {
	return l.version
}

func (l LiteralPacket) Type() PacketType {
	return Literal
}

var _ Packet = (*LiteralPacket)(nil)
