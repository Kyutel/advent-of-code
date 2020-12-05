#!/usr/bin/python3

def part1(puzzle_input):
    print(max([get_id_from_pass(x) for x in puzzle_input]))



def part2(puzzle_input):
    list_of_seats = [get_id_from_pass(x) for x in puzzle_input]
    missing_seats = list(set(range(0,1024)) - set(list_of_seats))
    for missing_seat in missing_seats:
        if (missing_seat + 1) in list_of_seats and (missing_seat - 1) in list_of_seats:
            print(missing_seat)

def get_input():
    INPUT_FILE_NAME = "input.txt"
    with open(INPUT_FILE_NAME, 'r') as file:
        data = file.read().split('\n')

    return data

def get_id_from_pass(boarding_pass):
    pass_as_binary = boarding_pass.replace('B','1').replace('F','0').replace('R','1').replace('L','0')

    return int(pass_as_binary , 2)
    

puzzle_input = get_input()
test_data = [
    'BFFFBBFRRR',
    'FFFBBBFRRR',
    'BBFFBBFRLL'
]

# for data in test_data:
#     print(get_id_from_pass(data))


part1(puzzle_input)
part2(puzzle_input)
