#!/usr/bin/python3.8
from functools import reduce
from math import sqrt
from copy import deepcopy

TOP = 0
RIGHT = 1
DOWN = 2
LEFT = 3

ROTATION = [TOP,RIGHT,DOWN,LEFT]
TILE_SIZE = 10

MONSTER_TEMPLATE = [
    '                  # ',
    '#    ##    ##    ###',
    ' #  #  #  #  #  #   '
]
MONSTER_TILES = 15

def part1(puzzle_input):
    tiles = {split[0][5:-1]:split[1:] for tile in puzzle_input if (split := tile.split('\n'))}
    borders = {}
    all_borders = set()
    duplicate_borders = set()
    for tile_id, tile in tiles.items():
        # print(tile_id, ': ', tile)
        borders[tile_id] = [tile[0]]
        borders[tile_id].append("".join(row[-1] for row in tile))
        borders[tile_id].append(tile[-1][::-1]) # reverse maybe
        borders[tile_id].append("".join(row[0] for row in tile)[::-1])


    for tile_id, border_list in borders.items():
        flipped_borders  = [border[::-1] for border in border_list]
        temp = flipped_borders[1]
        flipped_borders[1] = flipped_borders[3]
        flipped_borders[3] = temp
        borders[tile_id] += flipped_borders



    for tile_id in tiles.keys():
        for border in borders[tile_id]:
            if border not in all_borders:
                all_borders.add(border)
            else:
                duplicate_borders.add(border)
        # for border in flipped_borders[tile_id]:
        #     if border not in all_borders:
        #         all_borders.add(border)
        #     else:
        #         duplicate_borders.add(border)

    unique_borders = all_borders - duplicate_borders
    corners = []
    edges = []

    for tile_id in tiles.keys():
        unique_count = 0
        for border in borders[tile_id]:
            if border in unique_borders:
                unique_count +=1
        # for border in flipped_borders[tile_id]:
        #     if border in unique_borders:
        #         unique_count +=1
        
        if unique_count == 4:
            corners.append(tile_id)
        if unique_count == 2:
            edges.append(tile_id)

    answer = reduce((lambda x, y: int(x) * int(y)), corners)
    # print(edges)
    # print(corners)
    # print(answer)


    width = int(sqrt(len(tiles.keys())))
    print(width)
    current_tile = corners[0]
    picture = [[None]*width for i in range(width)]
    matched_on = [[None]*width for i in range(width)]
    picture[0][0] = current_tile

    remaining_tiles = list(tiles.keys())
    remaining_tiles.remove(current_tile)

    borders[current_tile] = borders[current_tile][:4]

    y = 0
    for x in range(1, width):
        for tile_id in remaining_tiles:
            if y == 0:
                if tile_id not in edges+corners:
                    continue

            matched = find_next_tile(current_tile, tile_id, borders, y, x, picture, remaining_tiles, matched_on)

            if matched:
                current_tile = tile_id
                break


    for y in range (1,width):
        for x in range(width):
            tile_above = picture[y-1][x]
            for tile_id in remaining_tiles:
                matched = find_next_tile(tile_above, tile_id, borders, y, x, picture, remaining_tiles, matched_on)


    answer = int(picture[0][0]) * int(picture[0][-1]) * int(picture[-1][0]) * int(picture[-1][-1])
    print(answer)

    rotation_count = get_rotation_count(matched_on[0][0], RIGHT)
    tiles[picture[0][0]] = rotate_tile_cw(tiles[picture[0][0]], rotation_count + 2)
    tiles[picture[0][0]] = flip_tile(tiles[picture[0][0]])

    y = 0
    for x in range(1, width):
        rotation_count = get_rotation_count(matched_on[y][x], LEFT)
        tile_id = picture[y][x] 
        # tiles[tile_id] = flip_tile(tiles[tile_id])
        if matched_on[y][x] > 3:
            # print(matched_on[y][x])
            tiles[tile_id] = flip_tile(tiles[tile_id])
        tiles[tile_id] = rotate_tile_cw(tiles[tile_id], rotation_count)

    for y in range (1,width):
        for x in range(width):
            rotation_count = get_rotation_count(matched_on[y][x], TOP)
            tile_id = picture[y][x] 
            if matched_on[y][x] > 3:
                tiles[tile_id] = flip_tile(tiles[tile_id])
            tiles[tile_id] = rotate_tile_cw(tiles[tile_id], rotation_count + 2) # UNCOMMENT FOR REAL
            # tiles[tile_id] = rotate_tile_cw(tiles[tile_id], rotation_count )


    picture.reverse() #UNCOMOMENT FOR REAL
    mega_tileset = []
    for y in range(width):
        tile_ids = picture[y]
        for row in range(len(tiles[tile_ids[0]])):
            line = "".join(tiles[ids][row] for ids in tile_ids)
            mega_tileset.append(line)
        # mega_tileset.append('')
    # print_tile(mega_tileset)

    return mega_tileset


def find_next_tile(current_tile, comparison, borders, y, x, picture, remaining_tiles, matched_on):
    current_tile_borders = set(borders[current_tile])
    comparison_border  = set(borders[comparison])
    matches = current_tile_borders.intersection(comparison_border)
    if matches:
        # print(matches)

        match_list = list(matches)

        if y == 0 and x == 1:
            index_of_matched = borders[current_tile].index(match_list[0])
            matched_on[0][0] = index_of_matched

        index_of_matched = borders[comparison].index(match_list[0])
        if index_of_matched > 3:
            borders[comparison] = borders[comparison][:4]
        else:
            borders[comparison] = borders[comparison][4:]
    
        matched_on[y][x] = index_of_matched

        picture[y][x] = comparison
        remaining_tiles.remove(comparison)
        return True
    return False

def get_rotation_count(matched_on_border_index, border_direction):
    rotation_count = 0
    current_index = matched_on_border_index % 4
        
    while current_index != border_direction:
        current_index = (current_index + 1) % 4
        rotation_count += 1

    return rotation_count

def print_tile(tile):
    for row in tile:
        print(row)

def flip_tile(tile):
    flipped_tile = []
    for row in tile:
        flipped_tile.append(row[::-1]) 

    return flipped_tile

def rotate_tile_cw(tile, rotation_count = 1):
    if rotation_count == 0:
        return tile

    rotated_tile = []
    for _ in range(rotation_count):
        for x in range(len(tile[0])):
            rotated_tile.append("".join(row[x] for row in tile)[::-1])
        tile = rotated_tile
        rotated_tile = []

    return tile

def print_dict(data_dict):
    for tile_id, data in data_dict.items():
        print(tile_id, ': ', data)
    

def part2(mega_tileset):
    picture = remove_borders(mega_tileset)
    # picture = flip_tile(picture)
    # picture = rotate_tile_cw(picture,3)
    # print_tile(picture)
    # print(search_cords_for_monster(picture, (16,0)))

    # print(vertical_search)
    # print(horizontal_search)
    # original = picture

    for _ in range(8):
        monster_count = search_picture_for_monster(picture)
        if monster_count != 0:
            break
        picture = rotate_tile_cw(picture)
        if _ == 3:
            picture = flip_tile(picture)

    hash_count = 0
    for row in picture:
        for pixel in row:
            if pixel == '#':
                hash_count += 1

    
    print(hash_count - (monster_count * MONSTER_TILES))


def search_picture_for_monster(picture):
    vertical_search = len(picture) - len(MONSTER_TEMPLATE) + 1
    horizontal_search = len(picture[0]) - len(MONSTER_TEMPLATE[0]) + 1
    monster_count = 0
    for y in range(vertical_search):
        for x in  range(horizontal_search):
            if search_cords_for_monster(picture, (y,x)):
                monster_count += 1

    return monster_count

def search_cords_for_monster(picture,cords):
    for y in range(len(MONSTER_TEMPLATE)):
        for x in range(len(MONSTER_TEMPLATE[0])):
            if MONSTER_TEMPLATE[y][x] == '#' and picture[y+cords[0]][x+cords[1]] != '#':
                return False

    return True

def remove_borders(mega_tileset):
    picture_size = len(mega_tileset[0]) // TILE_SIZE
    borderless = []
    for y in range(picture_size):
        for innerY in range((y*TILE_SIZE)+1,(y*TILE_SIZE)+9):
            line = ''
            for x in range(picture_size):
                line += mega_tileset[innerY][(x*TILE_SIZE)+1:(x*TILE_SIZE)+9]
            borderless.append(line)

    # print(borderless)
    return borderless

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n\n')

    return data


puzzle_input = get_input()
test_input = get_input('test-input.txt')

# mega_tileset = part1(test_input)
mega_tileset = part1(puzzle_input)
part2(mega_tileset)