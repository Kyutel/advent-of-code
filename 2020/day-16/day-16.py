#!/usr/bin/python3
import re
import copy
from functools import reduce

def part1(puzzle_input):
    valid_numbers = parse_ranges(puzzle_input[0])[0]

    nearby_tickets = parse_tickets(puzzle_input[2])

    invalid_nums = []
    for ticket in nearby_tickets:
        for number in ticket:
            if number not in valid_numbers:
                invalid_nums.append(number)

    # print(invalid_nums)
    print(sum(invalid_nums))


def part2(puzzle_input, test=False):
    valid_numbers, ranges = parse_ranges(puzzle_input[0])

    nearby_tickets = parse_tickets(puzzle_input[2])

    invalid_indexes = []
    for index, ticket in enumerate(nearby_tickets):
        for number in ticket:
            if number not in valid_numbers:
                invalid_indexes.append(index)
    
    valid_tickets = [ticket for index, ticket in enumerate(nearby_tickets) if index not in invalid_indexes]

    range_match = {}
    my_ticket = parse_tickets(puzzle_input[1])[0]
    remaining_indexes = list(range(len(my_ticket)))

    while remaining_indexes:
        for field, valid_numbers in ranges.items():
            if field in range_match:
                continue

            possible_indexes =  copy.copy(remaining_indexes)
            # print(possible_indexes)
            # print(valid_numbers)
            # while len(possible_indexes) > 1:
            for ticket in valid_tickets:
                for index in possible_indexes:
                    if ticket[index] not in valid_numbers:
                        # print()
                        # print(index, ticket[index])
                        possible_indexes.remove(index)
                        # break
                if len(possible_indexes) == 1:
                    break
            if len(possible_indexes) == 1:
                remaining_indexes.remove(possible_indexes[0])
                range_match[field] = possible_indexes[0]
    

    product = 1
    print(range_match)
    
    if not test:
        for field, index in range_match.items():
            if field.startswith('departure'):
                # print(field, index, my_ticket[index])
                product *= my_ticket[index]
        print(product)
    
    # for ticket in valid_tickets:
    #     print(ticket[17])


def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n\n')

    return data

def parse_ranges(raw_input):
    possible_numbers = set()
    ranges = {}
    for field in raw_input.split('\n'):
        numbers = re.findall(r'\d+', field)
        numbers = list(map(int,numbers))
        possible_numbers.update(range(numbers[0],numbers[1] + 1))
        possible_numbers.update(range(numbers[2],numbers[3] + 1))

        field_name = re.match(r'[a-zA-Z ]+', field)[0]
        ranges[field_name] = set(range(numbers[0],numbers[1] + 1))
        ranges[field_name].update(set(range(numbers[2],numbers[3] + 1)))

    # print(ranges)
    return possible_numbers, ranges

def parse_tickets(raw_input):
    unparsed_tickets = raw_input.split('\n')
    unparsed_tickets.pop(0)
    return [list(map(int,ticket.split(','))) for ticket in unparsed_tickets]

puzzle_input = get_input()
test_input = get_input('test-input.txt')
test_input2 = get_input('test-input2.txt')

part1(test_input)
# part1(puzzle_input)
# part2(test_input, True)
part2(test_input2, True)
part2(puzzle_input)