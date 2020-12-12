#!/usr/bin/python3

from collections import deque

direction_map = {
    'N': (-1, 0),
    'E': (0, 1),
    'S': (1, 0),
    'W': (0, -1)
}

def part1(navigation):
    location = [0,0]

    directions = deque([
        'E',
        'S',
        'W',
        'N'
    ])

    rotation_map = {
        'R': -1,
        'L': 1
    }

    for instruction in navigation:
        action = instruction[:1]
        value = int(instruction[1:])

        if action == 'F':
            action = directions[0]

        if action in direction_map:
            move_by_vector(location,direction_map[action], value)
        elif action == 'R' or action == 'L':
            directions.rotate(rotation_map[action] * value//90)

    print(location, get_manhattan_distance(location))

def part2(navigation):
    location = [0, 0]
    waypoint = [-1, 10]

    rotation_map = {
        'RY': 1,
        'LY': -1,
        'RX': -1,
        'LX': 1
    }

    for instruction in navigation:
        action = instruction[:1]
        value = int(instruction[1:])

        if action == 'F':
            move_by_vector(location, waypoint, value)
        elif action in direction_map:
            move_by_vector(waypoint, direction_map[action], value)
        elif action == 'R' or action == 'L':
            for _ in range(value//90):
                new_y = waypoint[1] * rotation_map[action + 'Y']
                new_x = waypoint[0] * rotation_map[action + 'X']
                waypoint = [new_y,new_x]

    print(location, get_manhattan_distance(location))

def move_by_vector(current_location, vector, scale):
    current_location[0] += vector[0] * scale
    current_location[1] += vector[1] * scale

def get_manhattan_distance(location):
    return abs(location[0]) + abs(location[1])

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data

test_data = [
    'F10',
    'N3',
    'F7',
    'R90',
    'F11'
]

puzzle_input = get_input()

part1(test_data)
part1(puzzle_input)
part2(test_data)
part2(puzzle_input)