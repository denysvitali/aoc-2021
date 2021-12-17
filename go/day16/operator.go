package main

import "math"

type OperatorPacket struct {
	i          byte
	l          uint32
	t          int
	version    int
	subPackets []Packet
}

func (o OperatorPacket) Evaluate() int {
	switch o.t {
	case 0:
		// Sum Packet
		sum := 0
		for _, s := range o.subPackets {
			sum += s.Evaluate()
		}
		return sum
	case 1:
		// Product Packet
		res := 1
		if len(o.subPackets) == 1 {
			return o.subPackets[0].Evaluate()
		}

		for _, s := range o.subPackets {
			res *= s.Evaluate()
		}
		return res
	case 2:
		// Minimum Packet
		if len(o.subPackets) == 0 {
			return 0
		}
		min := math.MaxInt
		for _, s := range o.subPackets {
			v := s.Evaluate()
			if v < min {
				min = v
			}
		}
		return min
	case 3:
		// Maximum Packet
		max := math.MinInt
		if len(o.subPackets) == 0 {
			return 0
		}
		for _, s := range o.subPackets {
			v := s.Evaluate()
			if v > max {
				max = v
			}
		}
		return max
	case 5:
		// Greater Than Packet
		if o.subPackets[0].Evaluate() > o.subPackets[1].Evaluate() {
			return 1
		}
		return 0
	case 6:
		// Less Than Packet
		if o.subPackets[0].Evaluate() < o.subPackets[1].Evaluate() {
			return 1
		}
		return 0
	case 7:
		// Equal Packet
		if o.subPackets[0].Evaluate() == o.subPackets[1].Evaluate() {
			return 1
		}
		return 0
	default:
		panic("invalid packet type")
	}

	return -1
}

func (o OperatorPacket) Version() int {
	return o.version
}

func (o OperatorPacket) Type() PacketType {
	return Operator
}

var _ Packet = (*OperatorPacket)(nil)
