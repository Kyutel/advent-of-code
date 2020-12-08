#!/usr/bin/python3
import copy

test_data = [
    'nop +0',
    'acc +1',
    'jmp +4',
    'acc +3',
    'jmp -3',
    'acc -99',
    'acc +1',
    'jmp -4',
    'acc +6'
]




def part1(puzzle_input):
    acc, pc = run_console(puzzle_input)
    print(acc)

def part2(puzzle_input):
    for i, instruction in enumerate(puzzle_input):
        program_copy = copy.deepcopy(puzzle_input)
        
        if instruction['operation'] == 'jmp':
            program_copy[i]['operation'] = 'nop'
        elif instruction['operation'] == 'nop':
            program_copy[i]['operation'] = 'nop'
        
        acc, pc = run_console(program_copy)
        if pc >= len(puzzle_input):
            print(acc)
            break


def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data

def parse_data(input_data):
    return [{'operation':instruction[0],'argument':int(instruction[1])} for instruction in (data.split(' ') for data in input_data)]


def run_console(program):
    accumulator = 0
    pc = 0
    end_program = False
    pc_history = [0]
    while not end_program:
        pc_history.append(pc)
        instruction = program[pc]

        if instruction['operation'] == "acc":
            accumulator += instruction['argument']
            pc += 1
        elif instruction['operation'] == "jmp":
            pc += instruction['argument']
        elif instruction['operation'] == "nop":
            pc += 1
    

        if pc in pc_history:
            end_program = True
        if pc >= len(program):
            end_program = True

    return accumulator,pc

puzzle_input = get_input()
# puzzle_input = parse_data(test_data)
puzzle_input = parse_data(puzzle_input)

# print(puzzle_input)
part1(puzzle_input)
part2(puzzle_input)