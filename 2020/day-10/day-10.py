#!/usr/bin/python3


def part1(puzzle_input):
    adapter_list = sorted(puzzle_input)
    # device_joltage = adapter_list[-1] + 3
    adapter_list.append(adapter_list[-1] + 3)
    adapter_list.insert(0,0)
    print(adapter_list)
    differences = {1:0, 2:0, 3:0}
    for index in range(1,len(adapter_list)):
        differences[adapter_list[index] - adapter_list[index - 1]] += 1 

    print(differences)
    print(differences[1] * differences[3])

def part2(puzzle_input):
    adapter_list = sorted(puzzle_input)
    # device_joltage = adapter_list[-1] + 3
    adapter_list.append(adapter_list[-1] + 3)
    adapter_list.insert(0,0)

    test = []
    for index in range(1, len(adapter_list)):
        difference = adapter_list[index] - adapter_list[index - 1]
        test.append(difference)
        

    print(test)

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data


puzzle_input = get_input()
puzzle_input = get_input("test-data.txt")

puzzle_input = list(map(int,puzzle_input))
part1(puzzle_input)
part2(puzzle_input)