package main

type OperatorPacket struct {
	i          byte
	l          uint32
	version    int
	subPackets []Packet
}

func (o OperatorPacket) Version() int {
	return o.version
}

func (o OperatorPacket) Type() PacketType {
	return Operator
}

var _ Packet = (*OperatorPacket)(nil)
