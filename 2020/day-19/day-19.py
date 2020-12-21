#!/usr/bin/python3.8
from re import match

def part1(rules, messages):
    rules.sort(key=lambda rule:int(rule.split(': ')[0]))
    rules = {int(split[0]):split[1] for rule in rules if(split := rule.split(': '))}
    rule_as_regex = get_rule_as_regex(rules[0], rules) + "$"

    matches = list(filter(lambda message: match(rule_as_regex,message), messages))
    print(len(matches))

def part2(rules, messages):
    rules.sort(key=lambda rule:int(rule.split(': ')[0]))
    rules = {int(split[0]):split[1] for rule in rules if(split := rule.split(': '))}
    rules[8] = '42 | 42 8'
    rules[11] = '42 31 | 42 11 31'
    rule_as_regex = get_rule_as_regex(rules[0], rules) + "$"

    matches = list(filter(lambda message: match(rule_as_regex,message), messages))
    print(len(matches))

recursive_store = {}
def get_rule_as_regex(rule, ruleset):
    # print(rule)
    if rule == '"a"':
        return 'a'
    elif rule == '"b"':
        return 'b'
    elif rule == '42 | 42 8':
        fourty_two = get_rule_as_regex(ruleset[42],ruleset)
        # eight =  get_rule_as_regex(ruleset[8],ruleset)
        # print(fourty_two)
        # print(eight)
        return '(' + fourty_two + ')+' 
        # return fourty_two + '|' + fourty_two
    elif rule == '42 31 | 42 11 31':
        fourty_two = '('+get_rule_as_regex(ruleset[42],ruleset)+')'
        thirty_one = '('+get_rule_as_regex(ruleset[31],ruleset)+')'
        # eleven = get_rule_as_regex(ruleset[11],ruleset)
        return f'({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}({fourty_two}{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})*{thirty_one})'
    else:
        expr = ''
        for symbol in rule.split(' '):
            if symbol.isnumeric():
                # print(symbol)
                expr += '('+  get_rule_as_regex(ruleset[int(symbol)],ruleset) + ')'
            else:
                expr += symbol
        return expr

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n\n')

    return data[0].split('\n'), data[1].split('\n')


rules, messages = get_input()
# rules, messages = get_input('test-input.txt')
# rules, messages = get_input('test-input2.txt')


part1(rules, messages)
part2(rules, messages)
