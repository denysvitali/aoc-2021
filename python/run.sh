#!/bin/bash

function usage() {
  echo "Usage: $0 DAY sample|input"
}

if [ $# -ne 2 ]; then
  usage
  exit 1
fi

DAY="$1"
SAMPLE_INPUT="$2"

case "$SAMPLE_INPUT" in
  sample)
    python3 main.py "day$DAY" "aoc/day$DAY/sample.txt"
    ;;
  input)
    python3 main.py "day$DAY" "aoc/day$DAY/input.txt"
    ;;
  *)
    echo "Unknown option $SAMPLE_INPUT" > /dev/stderr
    exit 1
  ;;
esac

