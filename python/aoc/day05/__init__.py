from typing import List, Dict, Tuple


class Line:
    x1 = -1
    x2 = -1
    y1 = -1
    y2 = -1

    def __init__(self, x1, y1, x2, y2):
        # (2,2) -> (2,1)
        # (9,4) -> (3, 4)
        if x1 > x2 or y1 > y2:
            # Swap
            self.x1 = x2
            self.x2 = x1

            self.y1 = y2
            self.y2 = y1
        else:
            self.x1 = x1
            self.y1 = y1

            self.x2 = x2
            self.y2 = y2

    def min(self) -> Tuple[int, int]:
        return min(self.x1, self.x2), min(self.y1, self.y2)

    def max(self) -> Tuple[int, int]:
        return max(self.x1, self.x2), max(self.y1, self.y2)

    def is_horizontal(self):
        return self.y2 == self.y1

    def is_vertical(self):
        return self.x2 == self.x1

    # bitmap is a 2D map with rows = y and columns = x
    def draw(self, bitmap: Dict[int, Dict[int, int]]):
        if self.is_vertical():
            for yc in range(self.y1, self.y2 + 1):
                if yc not in bitmap:
                    bitmap[yc] = dict()
                bitmap[yc][self.x1] = bitmap.get(yc).get(self.x1, 0) + 1
            return
        elif self.is_horizontal():
            if self.y1 not in bitmap:
                bitmap[self.y1] = dict()
            for xc in range(self.x1, self.x2 + 1):
                bitmap[self.y1][xc] = bitmap[self.y1].get(xc, 0) + 1
            return

        m = (self.y2 - self.y1) * 1.0 / (self.x2 - self.x1) * 1.0
        b = -(m * self.x1) + self.y1

        def f(x):
            return int(m * x + b)

        if self.x1 < self.x2:
            for xc in range(self.x1, self.x2 + 1):
                yc = f(xc)
                if yc not in bitmap:
                    bitmap[yc] = dict()
                bitmap[yc][xc] = bitmap.get(yc).get(xc, 0) + 1
        else:
            for xc in range(self.x2, self.x1 + 1):
                yc = f(xc)
                if yc not in bitmap:
                    bitmap[yc] = dict()
                bitmap[yc][xc] = bitmap.get(yc).get(xc, 0) + 1

    def __repr__(self):
        return f"Line=[({self.x1}, {self.y1}), ({self.x2}, {self.y2})]"


def parse_coordinate(input_str: str) -> List[int]:
    return list(map(lambda x: int(x), input_str.split(",")))


def parse_coords(input_file: str) -> List[Line]:
    with open(input_file) as f:
        f_lines: List[str] = list(map(lambda x: x.replace("\n", ""), f.readlines()))

    lines: List[Line] = list()

    for fLine in f_lines:
        first, second = fLine.split(" -> ")
        x1, y1 = parse_coordinate(first)
        x2, y2 = parse_coordinate(second)
        lines.append(Line(x1, y1, x2, y2))

    return lines


def draw_bitmap(bitmap: Dict[int, Dict[int, int]]):
    max_y = max(bitmap.keys())
    max_x = 0

    for y in bitmap:
        mx = max(bitmap[y].keys())
        if mx > max_x:
            max_x = mx

    for y in range(0, max_y + 1):
        for x in range(0, max_x + 1):
            if y not in bitmap:
                print(".", end="")
                continue
            if x not in bitmap[y]:
                print(".", end="")
                continue
            print(bitmap[y][x], end="")
        print()


def part_one(input_file: str) -> int:
    lines = parse_coords(input_file)
    bitmap = dict()
    for line in lines:
        if line.is_horizontal() or line.is_vertical():
            line.draw(bitmap)

    intersections = 0
    for y in bitmap:
        for x in bitmap[y]:
            if bitmap[y][x] >= 2:
                intersections += 1
    return intersections


def part_two(input_file: str) -> int:
    lines = parse_coords(input_file)
    bitmap = dict()
    for line in lines:
        line.draw(bitmap)

    intersections = 0
    for y in bitmap:
        for x in bitmap[y]:
            if bitmap[y][x] >= 2:
                intersections += 1
    return intersections


if __name__ == "__main__":
    assert part_one("sample.txt") == 5
    assert part_two("sample.txt") == 12

    assert part_one("input.txt") == 7318
    assert part_two("input.txt") == 19939
