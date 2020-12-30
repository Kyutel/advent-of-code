#!/usr/bin/python3.8

from copy import deepcopy

direction_map = {
    'e':(2, 0),
    'ne':(1, -1),
    'se':(1, 1),
    'w':(-2,0),
    'nw':(-1,-1),
    'sw':(-1, 1),
}

def part1(puzzle_input):
    pattern = [tile_path.replace('w','w ').replace('e','e ').split(' ')[0:-1] for tile_path in puzzle_input]
    flipped_tile = {}
    for path in pattern:
        location = [0, 0]
        for next_direction in path:
            location[0] += direction_map[next_direction][0]
            location[1] += direction_map[next_direction][1]
        location = tuple(location)
        current_tile_color = flipped_tile.get(location, 'white')
        if  current_tile_color == 'white':
            flipped_tile[location] = 'black'
        elif current_tile_color == 'black':
            flipped_tile[location] = 'white'

    print(sum(1 for color in flipped_tile.values() if color == 'black'))
    # print(flipped_tile)
    return flipped_tile

def part2(tile_map):
    sorted(tile_map)

    for _ in range(100):
        old_map = deepcopy(tile_map)
        for location, color in old_map.items():
            for direction in direction_map.values():
                location_to_check = (location[0] + direction[0], location[1] + direction[1])
                if old_map.get(location_to_check, '') == '':
                    tile_map[location_to_check] = get_new_color(old_map, 'white', location_to_check)
            tile_map[location] = get_new_color(old_map, color, location)
            # print(location, color)
        # print(sum(1 for color in tile_map.values() if color == 'black'))


    print()
    print(sum(1 for color in tile_map.values() if color == 'black'))

def get_new_color(old_map, color, location):
    black_count = 0
    for direction in direction_map.values():
        location_to_check = (location[0] + direction[0], location[1] + direction[1])
        if old_map.get(location_to_check, 'white') == 'black':
            black_count += 1

    if color == 'black':
        if black_count == 0 or black_count > 2:
            return 'white'
    elif color == 'white':
        if black_count == 2:
            return 'black'

    return color

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data


puzzle_input = get_input()
test_input = get_input('test-input.txt')

tile_map = part1(puzzle_input)
test_map = part1(test_input)

part2(tile_map)
# part2(test_map)
