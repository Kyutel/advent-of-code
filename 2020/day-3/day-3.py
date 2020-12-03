TREE = '#'

test_data = [
    '..##.......',
    '#...#...#..',
    '.#....#..#.',
    '..#.#...#.#',
    '.#...##..#.',
    '..#.##.....',
    '.#.#.#....#',
    '.#........#',
    '#.##...#...',
    '#...##....#',
    '.#..#...#.#'
]

def check_slope(input_map, right, down):
    x = 0
    tree_count = 0
    for y in range(0,len(input_map),down):
        row = input_map[y]
        if row[x] == TREE:
            tree_count+=1
        x  = (x + right) % len(row)
        
    return tree_count

def part1(puzzle_input):
    print(check_slope(puzzle_input,3 , 1))

def part2(puzzle_input):
    trees_multiplied = 1
    slopes_to_check = [
        {'right': 1, 'down': 1},
        {'right': 3, 'down': 1},
        {'right': 5, 'down': 1},
        {'right': 7, 'down': 1},
        {'right': 1, 'down': 2}
    ]

    for slope in slopes_to_check:
        trees = check_slope(puzzle_input, slope['right'], slope['down'])
        trees_multiplied *= trees

    print(trees_multiplied)

def get_input():
    INPUT_FILE_NAME = "input.txt"
    with open(INPUT_FILE_NAME, 'r') as file:
        data = file.read().split('\n')

    return data


puzzle_input = get_input()

# part1(test_data)
# part1(puzzle_input)
part2(test_data)
part2(puzzle_input)