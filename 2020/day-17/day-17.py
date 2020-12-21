#!/usr/bin/python3

from copy import deepcopy

ACTIVE = '#'
INACTIVE = '.'

def part1(puzzle_input):
    pocket_dimension = {0 : {index - 1:{index_2 - 1:row for index_2, row in enumerate(column)} for index, column in enumerate(puzzle_input)}}
    # print(pocket_dimension[0][-1][0])
    # print_pocket_dimention(pocket_dimension)

    for _ in range(6):
        simulate_cycle(pocket_dimension)

    # print_pocket_dimention(pocket_dimension)
    print(get_active(pocket_dimension))


def part2(puzzle_input):
    pocket_dimension = {0: {0 : {index - 1:{index_2 - 1:row for index_2, row in enumerate(column)} for index, column in enumerate(puzzle_input)}}}

    # print_pocket_dimention2(pocket_dimension)
    for _ in range(6):
        simulate_cycle2(pocket_dimension)
        # print_pocket_dimention2(pocket_dimension)
        # print(get_active2(pocket_dimension))
        # break

    print(get_active2(pocket_dimension))


def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data

def get_active2(pocket_dimension):
    count = 0
    for w in pocket_dimension.values():
        for z in w.values():
            for y in z.values():
                for x in y.values():
                    if x == ACTIVE:
                        count+= 1
    return count

def get_active(pocket_dimension):
    count = 0
    for z in pocket_dimension.values():
        for y in z.values():
            for x in y.values():
                if x == ACTIVE:
                    count+= 1

    return count
def print_pocket_dimention(pocket_dimension):
    for z_index, z in sorted(pocket_dimension.items()):
        print(f'Z={z_index}')
        for _, y in sorted(z.items()):
            print(''.join(y.values()))
        print()

def print_pocket_dimention2(pocket_dimension):
    for w_index, w in sorted(pocket_dimension.items()):
        for z_index, z in sorted(w.items()):
            print(f'Z={z_index} W={w_index}')
            for _, y in sorted(z.items()):
                print(''.join(y.values()))
            print()


def get_next_state2(location, pocket_dimension):
    count = 0
    for w in range(location[0] - 1, location[0] + 2):
        for z in range(location[1] - 1, location[1] + 2):
            for y in range(location[2] - 1, location[2] + 2):
                for x in range(location[3] - 1, location[3] + 2):
                    if (w,z,y,x) == location:
                        continue
                    if pocket_dimension.get(w,{z:{}}).get(z,{y:{}}).get(y, {}).get(x, INACTIVE) == ACTIVE:
                        count +=1
                    # print(pocket_dimension[z][y][x])
    w,z,y,x = location
    # print(location)
    # print(pocket_dimension)
    if pocket_dimension[w][z][y][x] == ACTIVE:
        if count == 2 or count == 3:
            return ACTIVE
        else:
            return INACTIVE
    else:
        if count == 3:
            return ACTIVE
        else:
            return INACTIVE

def get_next_state(location, pocket_dimension):
    count = 0
    for z in range(location[0] - 1, location[0] + 2):
        for y in range(location[1] - 1, location[1] + 2):
            for x in range(location[2] - 1, location[2] + 2):
                if (z,y,x) == location:
                    continue
                if pocket_dimension.get(z,{y:{}}).get(y, {}).get(x, INACTIVE) == ACTIVE:
                    count +=1
                # print(pocket_dimension[z][y][x])
    z,y,x = location
    # print(location)
    # print(pocket_dimension)
    if pocket_dimension[z][y][x] == ACTIVE:
        if count == 2 or count == 3:
            return ACTIVE
        else:
            return INACTIVE
    else:
        if count == 3:
            return ACTIVE
        else:
            return INACTIVE


def simulate_cycle(pocket_dimension):
    old_dimention = deepcopy(pocket_dimension)

    for z in range(min(old_dimention.keys()) - 1,max(old_dimention.keys()) + 2):
        if pocket_dimension.get(z) == None:
            pocket_dimension[z] = {}
        for y in range( min(old_dimention[0].keys()) - 1, max(old_dimention[0].keys()) + 2):
            if pocket_dimension[z].get(y) == None:
                pocket_dimension[z][y] = {}
            for x in range( min(old_dimention[0][0].keys()) - 1, max(old_dimention[0][0].keys()) + 2):
                if pocket_dimension[z][y].get(x) == None:
                    pocket_dimension[z][y][x] = INACTIVE

    # print(print_pocket_dimention(pocket_dimension))

    old_dimention = deepcopy(pocket_dimension)


    for z in range(min(old_dimention.keys()),max(old_dimention.keys()) + 1):
        for y in range( min(old_dimention[0].keys()), max(old_dimention[0].keys()) + 1):
            for x in range( min(old_dimention[0][0].keys()), max(old_dimention[0][0].keys()) + 1):
                    next_state = get_next_state((z,y,x), old_dimention)
                    pocket_dimension[z][y][x] = next_state


def simulate_cycle2(pocket_dimension):
    old_dimention = deepcopy(pocket_dimension)
    for w in range(min(old_dimention.keys()) - 1,max(old_dimention.keys()) + 2):
        if pocket_dimension.get(w) == None:
            pocket_dimension[w] = {}
        for z in range(min(old_dimention[0].keys()) - 1,max(old_dimention[0].keys()) + 2):
            if pocket_dimension[w].get(z) == None:
                pocket_dimension[w][z] = {}
            for y in range( min(old_dimention[0][0].keys()) - 1, max(old_dimention[0][0].keys()) + 2):
                if pocket_dimension[w][z].get(y) == None:
                    pocket_dimension[w][z][y] = {}
                for x in range( min(old_dimention[0][0][0].keys()) - 1, max(old_dimention[0][0][0].keys()) + 2):
                    if pocket_dimension[w][z][y].get(x) == None:
                        pocket_dimension[w][z][y][x] = INACTIVE

    # print(print_pocket_dimention(pocket_dimension))

    old_dimention = deepcopy(pocket_dimension)

    for w in range(min(old_dimention.keys()),max(old_dimention.keys()) + 1):
        for z in range(min(old_dimention[0].keys()),max(old_dimention[0].keys()) + 1):
            for y in range( min(old_dimention[0][0].keys()), max(old_dimention[0][0].keys()) + 1):
                for x in range( min(old_dimention[0][0][0].keys()), max(old_dimention[0][0][0].keys()) + 1):
                        next_state = get_next_state2((w,z,y,x), old_dimention)
                        pocket_dimension[w][z][y][x] = next_state

puzzle_input = get_input()
test_input = get_input('test-input.txt')

# part1(test_input)
# part1(puzzle_input)
# part2(test_input)
part2(puzzle_input)