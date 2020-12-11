#!/usr/bin/python3

import copy

FLOOR = '.'
EMPTY_SEAT = 'L'
TAKEN_SEAT = '#'

def part1(puzzle_input, expected = None):

    adjacency_grid = [[0 for x in puzzle_input] for y in puzzle_input]
    layout = [list(column[:]) for column in puzzle_input]

    stabilized = False
    while not stabilized:
        old_layout = copy.deepcopy(layout)
        apply_rules(layout, adjacency_grid, 4)
        if expected:
            next_expected = [list(column[:]) for column in expected.pop(0)]
            if layout != next_expected:
                print('Does not match expected layout')

        if old_layout == layout:
            stabilized = True

    print(count_seats(layout))


def part2(puzzle_input, expected = None):
    visibility_grid = [[0 for x in puzzle_input] for y in puzzle_input]
    layout = [list(column[:]) for column in puzzle_input]

    visibility_map = get_visibility_map(puzzle_input)

    stabilized = False
    while not stabilized:
        old_layout = copy.deepcopy(layout)
        apply_rules(layout, visibility_grid, 5, visibility_map)
        if expected:
            next_expected = [list(column[:]) for column in expected.pop(0)]
            if layout != next_expected:
                print('Does not match expected layout')

        if old_layout == layout:
            stabilized = True

    print(count_seats(layout))


def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data

def get_expected(expected_file_name):
    with open(expected_file_name, 'r') as file:
        data = file.read().split('\n\n')

    return [grid.split('\n') for grid in data]

def update_by_proximity(adjacency_grid, location, seat_change):
    for y in range(location[0] - 1, location[0] + 2) :
        for x in range(location[1] - 1,location[1] + 2):
            if y < 0 or y >= len(adjacency_grid) or x < 0 or x >= len(adjacency_grid[0]) or (y, x) == location:
                continue

            adjacency_grid[y][x] += seat_change



def update_by_sight(visibility_grid, visibility_map, location, seat_change):
    for visible_seats in visibility_map[location]:
        visibility_grid[visible_seats[0]][visible_seats[1]] += seat_change

def update_adjacent_seat_count(adjacency_grid, location, seat_change, sight_map):
    if sight_map:
        update_by_sight(adjacency_grid, sight_map, location, seat_change)
    else:
        update_by_proximity(adjacency_grid, location, seat_change)  


def find_visible_seats(layout, location):
    visible_seats = []
    #search left
    for x in range(location[1] -1, -1, -1):
        if layout[location[0]][x] != FLOOR:
            visible_seats.append((location[0],x)) 
            break

    #search right
    for x in range(location[1] + 1, len(layout[0])):
        if layout[location[0]][x] != FLOOR:
            visible_seats.append((location[0],x)) 
            break

    #search up
    for y in range(location[0] - 1, -1, -1):
        if layout[y][location[1]] != FLOOR:
            visible_seats.append((y,location[1])) 
            break

    #search down
    for y in range(location[0] + 1, len(layout)):
        if layout[y][location[1]] != FLOOR:
            visible_seats.append((y,location[1])) 
            break

    #search upright
    for y,x in zip(range(location[0] - 1, -1, -1), range(location[1] + 1, len(layout[0]))):
        if layout[y][x] != FLOOR:
            visible_seats.append((y,x)) 
            break
    #search upleft
    for y,x in zip(range(location[0] - 1, -1, -1), range(location[1] -1, -1, -1), ):
        if layout[y][x] != FLOOR:
            visible_seats.append((y,x)) 
            break
    #search downleft
    for y,x in zip(range(location[0] + 1, len(layout)), range(location[1] -1, -1, -1)):
        if layout[y][x] != FLOOR:
            visible_seats.append((y,x)) 
            break
    #search downright
    for y,x in zip(range(location[0] + 1, len(layout)), range(location[1] + 1, len(layout[0]))):
        if layout[y][x] != FLOOR:
            visible_seats.append((y,x)) 
            break

    return visible_seats

def get_visibility_map(layout):
    dictionary = {}
    for y in range(len(layout)):
        for x in range(len(layout[0])):
                if layout[y][x] != FLOOR:
                    dictionary[(y,x)] = find_visible_seats(layout,(y,x))

    return dictionary

def update_seat(layout, adjacency_grid, new_adjacency, location, tolerence, sight_map = None):
    y = location[0]
    x = location[1]
    seat = layout[y][x]
    if seat == EMPTY_SEAT:
        if adjacency_grid[y][x] == 0:
            seat = TAKEN_SEAT
            update_adjacent_seat_count(new_adjacency, (y, x), 1, sight_map)
    elif seat == TAKEN_SEAT:
        if adjacency_grid[y][x] >= tolerence:
            seat = EMPTY_SEAT
            update_adjacent_seat_count(new_adjacency, (y, x), -1, sight_map)
    elif seat == FLOOR:
        pass
    else:
        print('Somets Gone Wrong')

    return seat


def apply_rules(layout, adjacency_grid, tolerence, sight_map = None):
    new_adjacency = copy.deepcopy(adjacency_grid)
    for y in range(len(layout)):
        for x in range(len(layout[0])):
            layout[y][x] = update_seat(layout, adjacency_grid, new_adjacency, (y, x), tolerence, sight_map)

    for index in range(len(adjacency_grid)):
        adjacency_grid[index] = new_adjacency[index]

def count_seats(layout):
    count = 0
    for column in layout:
        for cell in column:
            if cell == TAKEN_SEAT:
                count += 1

    return count

puzzle_input = get_input()
test_input = get_input('test-input.txt')


part1(test_input, get_expected('test-expected.txt'))
part1(puzzle_input)
part2(test_input, get_expected('test-expected2.txt'))
part2(puzzle_input)