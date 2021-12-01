import sys
import os
from typing import List, Optional

def read_file_as_int(filename: str) -> List[int]:
  with open(filename) as f:
      lines = f.readlines()
  
  def remove_newline(input: str):
      return input.replace("\n", "")
  
  def as_int(input: str) -> int:
      return int(input)
  
  lines = map(remove_newline, lines)
  lines = map(as_int, lines)
  return list(lines)

if len(sys.argv) == 2:
  lines = read_file_as_int(sys.argv[1])
else:
  lines = read_file_as_int("sample.txt")

def part_one():
  prev : Optional[int] = None
  inc = 0
  dec = 0
  for i in lines:
    if prev != None:
      if i < prev:
        dec += 1
      elif i > prev:
        inc += 1
    prev = i
  return prev

def part_two():
  it1 = list()
  it2 = list()
  idx = 0
  inc = 0
  for i in lines:
    idx += 1
    if idx >= 4:
      a1 = lines[idx-4:idx-1]
      a2 = lines[idx-3:idx]
      print(a1, sum(a1), a2, sum(a2))
      if sum(a2) > sum(a1):
        inc += 1

  return inc


print(f"Part One: {part_one()}")
print(f"Part Two: {part_two()}")


