import copy
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
  mc : List[List[int]] = list()
  for line in lines:
      s = list(line)
      for i, x in enumerate(s):
          if len(mc) < i+1:
            mc.append(list())
          mc[i].append(int(x))

  gamma = ""
  epsilon = ""
  for x in mc:
    v = (sum(x)) > len(x)/2
    gamma += "1" if v else "0"
    epsilon += "0" if v else "1"
  gamma = int(gamma, 2)
  epsilon = int(epsilon, 2)
  return gamma * epsilon

def do_x(mc: List[List[Optional[int]]], isMcb: bool) -> int:
  output = ""
  for i1, x in enumerate(mc):
    v = (sum(x))
    if isMcb:
      cb = 1 if v >= (len(x) - v) else 0
    else:
      cb = 1 if v < (len(x) - v) else 0

    # Delete numbers not starting with v
    for i2, y in enumerate(mc[i1]):
        if y != cb:
            for e in mc:
                e[i2] = None
    # Cleanup None
    for idx, e in enumerate(mc):
        mc[idx] = list(filter(lambda x: x is not None, e))
    
    if len(mc[0]) == 1:
        for idx, v in enumerate(mc):
            for v2 in mc[idx]:
                output += "1" if v2 == 1 else "0"
        return int(output, 2)
        
  return -1 


def part_two(inputFile: str):
  lines = parse_file(inputFile)
  mc : List[List[int]] = list()
  for line in lines:
      s = list(line)
      for i, x in enumerate(s):
          if len(mc) < i+1:
            mc.append(list())
          mc[i].append(int(x))

  ogr = do_x(copy.deepcopy(mc), True)
  csr = do_x(copy.deepcopy(mc), False)
  print(ogr, csr)
  return ogr * csr
