#!/usr/bin/python3

def part1(puzzle_input):
    # print(puzzle_input)
    # ingredients = set()
    # alergens = set()
    possible_allergens = {}
    foods = []
    for row in puzzle_input:
        ingredients, allergens = row.split(' (') 
        ingredients = ingredients.split(' ')
        allergens = allergens[9:-1].split(', ')
        # print(ingredients)
        # print(allergens)
        for allergen in allergens:
            if possible_allergens.get(allergen):
                possible_allergens[allergen] = possible_allergens[allergen].intersection(ingredients)
            else:
                possible_allergens[allergen] = set(ingredients)

        foods.append((ingredients,allergens))

    # print(possible_allergens)
    # eliminate_possibilities(possible_allergens)
    all_allergens = set()
    for allergen, possibilities in possible_allergens.items():
        all_allergens.update(possibilities)
    # print_list(foods)
    count = 0
    for food in foods:
        for ingredients in food[0]:
            if ingredients not in all_allergens:
                count += 1

    print(count)
    return possible_allergens

def print_list(list_to_print):
    for row in list_to_print:
        print(row)


def part2(possible_allergens):
    confirmed = {}
    print(possible_allergens)
    while len( possible_allergens) != len(confirmed):
        for allergen, possibilities in possible_allergens.items():
            if allergen not in confirmed and len(possibilities) == 1:
                confirmed_allergen = possibilities.pop()
                confirmed[allergen] = confirmed_allergen
                for other_allergen in possible_allergens.keys():
                    if allergen != other_allergen:
                        possible_allergens[other_allergen].discard(confirmed_allergen)

    print(confirmed)
    confirmed = ",".join(confirmed[value] for value in sorted(confirmed))
    print(confirmed)

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data


puzzle_input = get_input()
test_input = get_input('test-input.txt')


possible_allergens = part1(puzzle_input)
# possible_allergens = part1(test_input)
part2(possible_allergens)
