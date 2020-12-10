#!/usr/bin/python3

def part1(puzzle_input, preamble_length):
    # preamble = puzzle_input[:preamble_length]
    comparison_point = 0
    number_list = puzzle_input[preamble_length:]
    for number in number_list:
        sum_found = False
        comparison_values = puzzle_input[comparison_point:comparison_point+ preamble_length]
        for comparison in comparison_values:
            if (number - comparison) in comparison_values and comparison!= number/2:
                sum_found = True
        comparison_point += 1
        if sum_found == False:
            print(number)
            return number

def part2(puzzle_input, target_value):
    for index, number in enumerate(puzzle_input):
        current_sum = number
        chain_size = 1
        while index + chain_size < len(puzzle_input) and current_sum < target_value:
            chain_size += 1
            current_sum += puzzle_input[index + chain_size - 1]

        if current_sum == target_value and chain_size > 1:
            number_range = puzzle_input[index: index + chain_size]
            print(min(number_range) + max(number_range))
            return

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data



puzzle_input = get_input()
test_input = get_input("test-data.txt")

test_input = list(map(int,test_input))
puzzle_input = list(map(int,puzzle_input))
# test_result = part1(test_input, 5)
part_1_result = part1(puzzle_input, 25)

# part2(test_input, test_result)
part2(puzzle_input, part_1_result)