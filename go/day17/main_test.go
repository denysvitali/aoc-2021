package main

import (
	"github.com/sirupsen/logrus"
	"github.com/stretchr/testify/assert"
	"os"
	"testing"
)

func TestMain(m *testing.M) {
	logrus.SetLevel(logrus.DebugLevel)
	os.Exit(m.Run())
}

func TestMotion(t *testing.T){
	m := NewMotion(7,2)
	assert.Equal(t, Coord{x: 0, y: 0}, m.pos)
	m.step()
	assert.Equal(t, Coord{x: 7, y: 2}, m.pos)
	m.step()
	assert.Equal(t, Coord{x: 13, y: 3}, m.pos)
	m.step()
	assert.Equal(t, Coord{x: 18, y: 3}, m.pos)
	m.step()
	assert.Equal(t, Coord{x: 22, y: 2}, m.pos)
	m.step()
	assert.Equal(t, Coord{x: 25, y: 0}, m.pos)
	m.step()
	assert.Equal(t, Coord{x: 27, y: -3}, m.pos)
	m.step()
	assert.Equal(t, Coord{x: 28, y: -7}, m.pos)
}

func TestSamplePartOne(t *testing.T){
	v := part1("input/sample.txt")
	assert.Equal(t, 45, v)
}

func TestSamplePartTwo(t *testing.T){
	v := part2("input/sample.txt")
	assert.Equal(t, 112, v)
}

func TestInputPartOne(t *testing.T){
	v := part1("input/input.txt")
	assert.Equal(t, 10296, v)
}

func TestInputPartTwo(t *testing.T){
	v := part2("input/input.txt")
	assert.Equal(t, 2371, v)
}


