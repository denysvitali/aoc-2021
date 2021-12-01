import sys
import os
from typing import List, Optional
from ..utils import read_file_as_int

def part_one(inputFile: str):
  lines = read_file_as_int(inputFile)
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

def part_two(inputFile: str):
  lines = read_file_as_int(inputFile)
  idx = 0
  inc = 0
  for _ in lines:
    idx += 1
    if idx >= 4:
      a1 = lines[idx-4:idx-1]
      a2 = lines[idx-3:idx]
      if sum(a2) > sum(a1):
        inc += 1

  return inc
