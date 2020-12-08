#!/usr/bin/python3
import pprint

def part1(bag_dictionary):
    BAG_TO_FIND = "shiny gold"
    bag_list = find_bags_that_contain_bag(bag_dictionary, BAG_TO_FIND)
    print(len(bag_list))


def part2(bag_dictionary):
    BAG_TO_SEARCH = "shiny gold"
    bag_count = count_bags_inside_bag(bag_dictionary, BAG_TO_SEARCH)
    print(bag_count - 1)

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data


def find_bags_that_contain_bag(bag_dictionary, bag_to_find):
    list_of_bags = set()
    for outer, inner in bag_dictionary.items():
        if bag_to_find in inner.keys():
            list_of_bags.add(outer)
            list_of_bags.update(find_bags_that_contain_bag(bag_dictionary, outer))

    return list_of_bags    

def count_bags_inside_bag(bag_dictionary, bag_to_search):
    count = 1
    inner_bags = bag_dictionary[bag_to_search]
    if not inner_bags:
        return 1
    else:
        for inner_color, number_of_bags in inner_bags.items():
            count += number_of_bags * count_bags_inside_bag(bag_dictionary, inner_color)

    if(bag_to_search) == 'vibrant plum':
        print('plum ' + str(count))
    # print(sum(inner_bags.values()))

    return count

def parse_input(puzzle_input):
    parsed_input = {}
    for raw_info in puzzle_input:
        bag_info = raw_info.split(' ')
        outer_bag_color = f'{bag_info[0]} {bag_info[1]}'
        if bag_info[4] == "no":
            parsed_input[outer_bag_color] = {}
        else:
            # parsed_input[outer_bag_color] = [{'count':bag_info[index], 'colour':  f'{bag_info[index + 1]} {bag_info[index + 2]}'}  for index in range(4,len(bag_info),4)]
            parsed_input[outer_bag_color] = {f'{bag_info[index + 1]} {bag_info[index + 2]}': int(bag_info[index])  for index in range(4,len(bag_info),4)}

    return parsed_input

puzzle_input = get_input()
test_input = get_input('test-input.txt')
bag_dictionary = parse_input(test_input)
bag_dictionary = parse_input(puzzle_input)

# print(bag_dictionary)
pp = pprint.PrettyPrinter()
# pp.pprint(bag_dictionary)

# part1(bag_dictionary)
part2(bag_dictionary)