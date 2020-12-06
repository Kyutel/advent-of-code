#!/usr/bin/python3

def part1(puzzle_input):
    print(sum(len(set(answers.replace('\n',""))) for answers in puzzle_input))


def part2(puzzle_input):
    group_answers = [group_answers.split('\n') for group_answers in puzzle_input]
    print(sum([len(set.intersection(*[set(answer) for answer in answers])) for answers in group_answers]))
   


def get_input():
    INPUT_FILE_NAME = "input.txt"
    with open(INPUT_FILE_NAME, 'r') as file:
        data = file.read().split('\n\n')

    return data


puzzle_input = get_input()

part1(puzzle_input)
part2(puzzle_input)