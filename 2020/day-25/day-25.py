#!/usr/bin/python3.8

def part1(puzzle_input):
    public_keys = list(map(int, puzzle_input))

    card_loop = get_loop_number(public_keys[0])
    door_loop = get_loop_number(public_keys[1])
    print(card_loop, door_loop)

    print(pow(public_keys[0], door_loop) % 20201227)

def get_loop_number(public_key, subject_number = 7):
    value = 1
    loop_number = 0
    while value != public_key:
        value = (value * subject_number) % 20201227

        loop_number += 1

    return loop_number

def part2(puzzle_input):
    pass

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data


puzzle_input = get_input()
test_input = [
    '5764801',
    '17807724'
]

part1(puzzle_input)
# part1(test_input)
# part2(puzzle_input)
part2(test_input)
