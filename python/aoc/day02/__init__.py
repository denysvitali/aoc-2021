import sys
import os
from typing import List, Optional
from ..utils import read_file_as_int

def remove_newline(line: str) -> str:
    return line.replace("\n", "")

def parse_file(inputFile: str):
    with open(inputFile) as f:
        return list(map(remove_newline, f.readlines()))

def get_dir_value(line: str):
    return line.split(" ")

def part_one(inputFile: str):
  lines = parse_file(inputFile)
  depth = 0
  forward = 0
  for line in lines:
      dv = get_dir_value(line)
      d = dv[0]
      v = int(dv[1])
      if d == "forward":
          forward += v
      elif d == "down":
          depth += v
      elif d == "up":
          depth -= v
  return forward * depth

def part_two(inputFile: str):
  lines = parse_file(inputFile)
  depth = 0
  forward = 0
  aim = 0
  for line in lines:
      dv = get_dir_value(line)
      print(dv)
      d = dv[0]
      v = int(dv[1])
      if d == "forward":
          forward += v
          depth += aim * v
          print(f"inc={aim*v}")
      elif d == "down":
          aim += v
          print(f"depth + {v}")
      elif d == "up":
          aim -= v
  print(forward, depth)
  return forward * depth
