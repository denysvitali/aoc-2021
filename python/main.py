#!/usr/bin/env python
# -*- coding: utf-8 -*-
import sys
import importlib

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} day input")
        sys.exit(1)
    package = sys.argv[1]
    inputFile = sys.argv[2]
    pkgName = f"aoc.{package}"

    mod = importlib.import_module(pkgName)
    partOne = mod.part_one(inputFile)
    partTwo = mod.part_two(inputFile)

    print(f"Part One: {partOne}")
    print(f"Part Two: {partTwo}")
