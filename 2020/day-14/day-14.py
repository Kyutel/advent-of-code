#!/usr/bin/python3

def part1(puzzle_input):
    mask1 = ''
    mask0 = ''
    memory = {}
    for code in puzzle_input:
        code = code.split(' = ')
        if code[0] == 'mask':
            mask1 = int(code[1].replace('X', '0'),2)
            mask0 = int(code[1].replace('X', '1'),2)
        else:
            value = int(code[1])
            masked_value = (value | mask1)  & mask0
            memory[code[0]] = masked_value
        # print(code)
    # print(memory)
    print(sum(memory.values()))


def part2(puzzle_input):
    memory = {}
    for code in puzzle_input:
        code = code.split(' = ')
        if code[0] == 'mask':
            mask = code[1]
        else:
            address = int(code[0][4:-1])
            masked_address = apply_mask_to_address(address, mask)
            possible_addresses = get_addresses('', list(masked_address))
            possible_addresses = [int(address,2) for address in possible_addresses]
            for address in possible_addresses:
                memory[address] = int(code[1])

    # print(memory)
    print(sum(memory.values()))


def apply_mask_to_address(address,mask):
    address = list(format(address, '#038b'))
    address = address[2:]

    for index in range(len(mask)):
        # print(index, address[-index], mask[-index])
        address[index] = mask[index] if mask[index] != '0' else address[index]

    return "".join(address)


def get_addresses(current_address, remaining):
    # print(current_address, remaining)
    if not remaining:
        return [current_address]
    next_letter = remaining.pop(0)
    if next_letter == 'X':
        v1 = get_addresses(current_address, ['0'] + remaining)
        v2 = get_addresses(current_address, ['1'] + remaining)

        return v1 + v2
    else:
        current_address += next_letter
        address = get_addresses(current_address,remaining)
        return address

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data

# def parsed_input(puzzle_input):
#     return parsed_input

puzzle_input = get_input()
# puzzle_input = get_input('test-input.txt')


test_input_2 = get_input('test-input2.txt')
# puzzle_input = parsed_input(puzzle_input)

# part1(puzzle_input)
part2(test_input_2)
part2(puzzle_input)