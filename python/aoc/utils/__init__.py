#!/usr/bin/env python
# -*- coding: utf-8 -*-
from typing import List

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
