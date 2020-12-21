#!/usr/bin/python3

def part1(puzzle_input):
    evaluations = []

    for expression in puzzle_input:
        edited_expression = expression.replace('(','( ').replace(')',' )')
        acc = 0
        next_operator = '+'
        stack = []
        for value in edited_expression.split():
            if value.isnumeric():
                acc = do_calculation(acc, next_operator, int(value))
            elif value == '(':
                stack.append(acc)
                stack.append(next_operator)
                acc = 0
                next_operator = '+'
            elif value == ')':
                next_operator = stack.pop()
                acc = do_calculation(acc, next_operator, stack.pop())
            else:
                next_operator = value
            # print(acc)
        # print(acc)
        evaluations.append(acc)

    print(sum(evaluations))

def do_calculation(acc, operator, value):
    if operator == '+':
        return acc + value
    else:
        return acc * value

def part2(puzzle_input):
    evaluations = []

    for expression in puzzle_input:
        edited_expression = expression.replace('(','( ').replace(')',' )')
        acc = 0
        next_operator = '+'
        stack = ['#']
        postfix = ''
        for value in edited_expression.split():
            if value.isnumeric():
                postfix += value + ' '
                # stack.append()
                # acc = do_calculation(acc, next_operator, int(value))
            elif value == '(':
                stack.append(value)
                # acc = 0
                # next_operator = '+'
            # elif value == '+':
            #     stack.append(value)
            elif value == ')':
                postfix += reduce_stack(stack) + ' '
                stack.pop()
                # acc = do_calculation(acc, next_operator, stack.pop())
            else:
                while len(stack) > 1 and prec(value) <= prec(stack[-1]):
                    postfix += stack.pop() + ' '
                stack.append(value)
            # print(stack,'val: ', value, ', psfx: ', postfix)
        while len(stack) > 1:
            postfix += stack.pop() + ' '
        # print(stack, ', psfx: ', postfix)
        # print(acc)
        new_stack = []
        # print(postfix.split())
        for val in postfix.split():
            if val.isnumeric():
                new_stack.append(int(val))
            else:
                calc = do_calculation(new_stack.pop(), val,new_stack.pop())
                new_stack.append(calc)
                # print(calc)
        # print(new_stack)
        evaluations.append(new_stack[0])

    print(sum(evaluations))


def prec(operator):
    if operator == '+':
        return 2
    elif operator == '*':
        return 1
    else:
        return 0

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data

def reduce_stack(stack):
    postfix = ''
    while len(stack) > 1 and stack[-1] != '(':
        postfix += stack.pop() + ' '
    return postfix


puzzle_input = get_input()

test_data = [
    '1 + 2 * 3 + 4 * 5 + 6',
    '1 + (2 * 3) + (4 * (5 + 6))',
    '2 * 3 + (4 * 5)',
    '5 + (8 * 3 + 9 + 3 * 4 * 3)',
    '5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))',
    '((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2'
]

# part1(test_data)
part1(puzzle_input)
part2(test_data)
part2(puzzle_input)