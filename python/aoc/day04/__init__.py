from typing import List, Optional


def filter_none(r: List[Optional[int]]) -> List[int]:
    return list(filter(None, r))


class Board:
    content: List[List[int]] = list()
    won = False

    def __init__(self, content: List[List[int]]):
        self.content = content

    def check_win(self):
        if self.won:
            return True
        if self.wins():
            self.won = True
        return self.won

    def score(self) -> int:
        return sum(list(map(lambda x: sum(filter_none(x)), self.content)))

    def print(self):
        for r in self.content:
            for c in r:
                print("%3s" % (c if c is not None else "X"), end=" ")
            print("\n")

    def __repr__(self) -> str:
        output = ""
        for r in self.content:
            for c in r:
                output += "%3s " % (c if c is not None else "X")
            output += "\n"
        return output

    def wins(self) -> bool:
        for i, r in enumerate(self.content):
            if sum(filter_none(r)) == 0:
                return True
        for j in range(0, len(self.content[0])):
            s = 0
            for r in self.content:
                if r[j] is not None:
                    s += r[j]
            if s == 0:
                return True
        return False


def remove_newline(line: str) -> str:
    return line.replace("\n", "")


def parse_file(input_file: str):
    with open(input_file) as f:
        return list(map(remove_newline, f.readlines()))


def get_dir_value(line: str):
    return line.split(" ")


def part_one(input_file: str):
    lines = parse_file(input_file)
    draw = list(map(lambda x: int(x), lines[0].split(",")))
    rest = lines[2:]
    raw_boards = "\\n".join(rest)
    raw_boards = raw_boards.split("\\n\\n")
    boards: List[Board] = list()
    for c in raw_boards:
        c = map(lambda x: x.strip(), c.split("\\n"))
        c = map(lambda x: x.replace("  ", " "), c)
        c = map(lambda x: x.split(" "), c)
        c = map(lambda x: list(map(lambda y: int(y), x)), c)
        b = Board(list(c))
        boards.append(b)

    for d in draw:
        # Drawing d
        for b in boards:
            for idx, r in enumerate(b.content):
                b.content[idx] = [x if x != d else None for x in b.content[idx]]
            if b.check_win():
                s = b.score()
                return s * d
    return -1


def part_two(input_file: str):
    lines = parse_file(input_file)
    draw = list(map(lambda x: int(x), lines[0].split(",")))
    rest = lines[2:]
    raw_boards = "\\n".join(rest)
    raw_boards = raw_boards.split("\\n\\n")
    boards: List[Board] = list()
    for c in raw_boards:
        c = map(lambda x: x.strip(), c.split("\\n"))
        c = map(lambda x: x.replace("  ", " "), c)
        c = map(lambda x: x.split(" "), c)
        c = map(lambda x: list(map(lambda y: int(y), x)), c)
        boards.append(Board(list(c)))

    for d in draw:
        # Drawing d
        for b in boards:
            if b.won:
                continue
            for idx, r in enumerate(b.content):
                b.content[idx] = list(map(lambda x: None if x == d else x, r))
            if b.check_win():
                not_won = list(filter(lambda x: not x.won, boards))
                if len(not_won) == 0:
                    s = b.score()
                    return s * d
    return -1


if __name__ == "__main__":
    assert part_one("sample.txt") == 4512
    assert part_two("sample.txt") == 1924
    assert part_one("input.txt") == 25023
    assert part_two("input.txt") == 2634
