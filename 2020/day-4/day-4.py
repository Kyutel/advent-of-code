#!/usr/bin/python3
import re

def part1(passports):
    
    valid_passports = 0

    for passport in passports:
        if len(passport) == 8:
            valid_passports += 1
        elif len(passport) == 7 and passport.get('cid') == None:
            valid_passports += 1

    print(valid_passports)


def part2(passports):
    valid_passports = 0

    for passport in passports:
        if len(passport) == 8 and validate_passport(passport):
            valid_passports += 1
        elif len(passport) == 7 and passport.get('cid') == None and validate_passport(passport):
            valid_passports += 1

    print(valid_passports)

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n\n')

    return data

def parse_data_into_passports(puzzle_input):
    passports = []
    for data in puzzle_input:
        split_details = re.split(r"[\s]",data)
        passport = {}
        for field in split_details:
            passport[field[:3]] = field[4:]
        
        # print(passport)
        passports.append(passport)

    return passports

def validate_passport(passport):

    return validate_birth_year(passport['byr']) and validate_issue_year(passport['iyr']) \
           and validate_experation_year(passport['eyr']) and validate_height(passport['hgt'])\
           and validate_hair_color(passport['hcl']) and validate_eye_color(passport['ecl'])\
           and validate_passport_id(passport['pid'])


def validate_birth_year(year):
    return len(year) == 4 and int(year) >= 1920 and int(year) <= 2002

def validate_issue_year(year):
    return len(year) == 4 and int(year) >= 2010 and int(year) <= 2020

def validate_experation_year(year):
    return len(year) == 4 and int(year) >= 2020 and int(year) <= 2030

def validate_height(height):
    measurement = height[-2:]
    value = int(height[:-2])
    if measurement == 'cm' and value >= 150 and value <= 193:
        return True
    elif measurement == 'in' and value >= 59 and value <= 76:
        return True
    else:
        return False

def validate_hair_color(color):
    return re.match(r'^#(\d|[a-f]){6}$', color) != None

def validate_eye_color(color):
    valid_colors = ['amb','blu','brn','gry','grn','hzl','oth']
    return color in valid_colors


def validate_passport_id(id):
    return re.match(r'^\d{9}$', id) != None

test_input = parse_data_into_passports(get_input("test-data.txt"))
valid_passports = parse_data_into_passports(get_input("valid-passports.txt"))
puzzle_input = parse_data_into_passports(get_input())

# print(test_input)
# part1(test_input)
part1(puzzle_input)
# part2(valid_passports)
part2(puzzle_input)
