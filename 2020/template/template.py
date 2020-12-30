#!/usr/bin/python3.8

def part1(puzzle_input):
    pass


def part2(puzzle_input):
    pass

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data


puzzle_input = get_input()
test_input = get_input('test-input.txt')

# part1(puzzle_input)
part1(test_input)
# part2(puzzle_input)
part2(test_input)
