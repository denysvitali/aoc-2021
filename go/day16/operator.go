package main

type OperatorPacket struct {
	i          byte
	l          uint32
	subPackets []Packet
}

func (o OperatorPacket) Type() PacketType {
	return Operator
}

var _ Packet = (*OperatorPacket)(nil)
