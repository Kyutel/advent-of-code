#!/usr/bin/python3

def part1(puzzle_input):
    turn = 1
    when_spoken = {}
    # last_number = 0
    for number in puzzle_input:
        when_spoken[number] = [turn]
        turn+=1

    # print(when_spoken)
    last_spoken = puzzle_input[-1]
    for turn in range(turn, 2021):
        # print(last_spoken)
        # print(len(when_spoken[last_spoken]))
        speak = -1
        if len(when_spoken[last_spoken]) == 1:
            speak = 0
        elif len(when_spoken[last_spoken]) > 1:
            speak = when_spoken[last_spoken][-1] - when_spoken[last_spoken][-2]
        else:
            print('somet wrong')
        last_spoken = speak

        if when_spoken.get(speak):
            when_spoken[speak] += [turn]
        else:
            when_spoken[speak] = [turn]

    # print(when_spoken)
    print(last_spoken)

def part2(puzzle_input):
    turn = 1
    when_spoken = {}
    # last_number = 0
    for number in puzzle_input:
        when_spoken[number] = [turn]
        turn+=1

    # print(when_spoken)
    last_spoken = puzzle_input[-1]
    for turn in range(turn, 30000001):
        # print(last_spoken)
        # print(len(when_spoken[last_spoken]))
        speak = -1
        if len(when_spoken[last_spoken]) == 1:
            speak = 0
        elif len(when_spoken[last_spoken]) > 1:
            speak = when_spoken[last_spoken][-1] - when_spoken[last_spoken][-2]
        else:
            print('somet wrong')
        last_spoken = speak

        if when_spoken.get(speak):
            when_spoken[speak] += [turn]
        else:
            when_spoken[speak] = [turn]

    # print(when_spoken)
    print(last_spoken)

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split(',')

    return data


test_input = [0,3,6]
# test_input = [1,3,2]
# test_input = [2,3,1]
# test_input = [3,2,1]
# test_input = [3,1,2]
puzzle_input = get_input()
puzzle_input = list(map(int,puzzle_input))
# print(puzzle_input)
# part1(test_input)
# part1(puzzle_input)
# part2(test_input)
part2(puzzle_input)