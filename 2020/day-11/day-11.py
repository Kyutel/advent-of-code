#!/usr/bin/python3

FLOOR = '.'
EMPTY_SEAT = 'L'
TAKEN_SEAT = '#'

def part1(puzzle_input, expected = None):
    # print(puzzle_input)
    # print(expected)


def part2(puzzle_input):
    pass

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data

def get_expected(expected_file_name):
    with open(expected_file_name, 'r') as file:
        data = file.read().split('\n\n')

    return [grid.split('\n') for grid in data]

def get_adjacent_seat_count(layout, location):
    


# puzzle_input = get_input()
test_input = get_input('test-input.txt')


part1(test_input, get_expected('test-expected.txt'))
# part2(puzzle_input)