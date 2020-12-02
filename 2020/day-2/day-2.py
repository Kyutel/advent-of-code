import re

test_input  = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]

def part1(puzzle_input):
    count = 0
    for row in puzzle_input:
        text = row[2]
        letter_count = text.count(row[1])

        if letter_count >= row[0]['min'] and letter_count <= row[0]['max']:
            # print(f'{text} - {letter_count}')
            count += 1 

    print(count)

def part2(puzzle_input):
    count = 0
    for row in puzzle_input:
        text = row[2]
        match_count = 0
        if text[row[0]['min'] - 1] == row[1]:
            match_count+= 1
        if text[row[0]['max'] - 1] == row[1]:
            match_count += 1

        if match_count == 1:
            count += 1

    print(count)

def parse_input(puzzle_input):
    parsed = []
    for row in puzzle_input:
        parts = row.split(' ')
        counts = parts[0].split('-')
        min_count = int(counts[0])
        max_count = int(counts[1])
        parts[0] = {'min':min_count, 'max':max_count}
        letter = parts[1][:1]
        parts[1] = letter
        parsed += [parts]


    return parsed

def get_input():
    INPUT_FILE_NAME = "input.txt"
    with open(INPUT_FILE_NAME, 'r') as file:
        data = file.read().split('\n')

    return data


puzzle_input = parse_input(get_input())
# puzzle_input = parse_input(test_input)


part1(puzzle_input)
part2(puzzle_input)

