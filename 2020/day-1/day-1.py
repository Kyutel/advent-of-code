

def part1(puzzle_input):
    # print(puzzle_input)
    under = [int(x) for x in puzzle_input if int(x) < (2020/2) ]
    over =  [int(x) for x in puzzle_input if int(x) >= (2020/2) ]

    for x in under:
        for y in over:
            # print(f'{x}, {y}')
            if x + y == 2020:
                print(x*y)


def part2(puzzle_input):
    temp = []
    for x in puzzle_input:
        for y in puzzle_input:
            for z in puzzle_input:
                if int(x) + int(y) + int(z) == 2020:
                    print(int(x)*int(y)*int(z))
                    return

def get_input():
    INPUT_FILE_NAME = "input.txt"
    with open(INPUT_FILE_NAME, 'r') as file:
        data = file.readlines()

    return data


puzzle_input = get_input()
part1(puzzle_input)

part2(puzzle_input)