#!/usr/bin/python3.8
from math import ceil
from functools import reduce

def part1(expected_time, bus_ids):
    lowest_times = {}
    for bus in bus_ids:
        if bus != 'x':
            bus = int(bus)
            lowest_time = ceil(expected_time / bus) * bus
            lowest_times[bus] =  lowest_time

    target_bus = min(lowest_times.items(), key=lambda lowest_time: lowest_time[1])
    print((target_bus[1] - expected_time) * target_bus[0])

def part2(bus_ids):
    t = 0
    filtered_busses = {int(bus_id):index for index, bus_id in enumerate(bus_ids) if bus_id != 'x'}
    increment = int(bus_ids[0])

    # print(filtered_busses)
    found = False
    lst = [increment]
    while not found:
        t += increment
        found = True
        for bus in filtered_busses.items():
            if (t + bus[1] ) % bus[0] != 0:
                found = False
                break
            elif bus[0] not in lst:
                increment *= bus[0]
                lst.append(bus[0])

    print(t)

def part2_chinese_remainder(bus_ids):
        bus_ids = list(reversed(bus_ids))
        filtered_busses = {int(bus_id):index for index, bus_id in enumerate(bus_ids) if bus_id != 'x'}
        # print(filtered_busses)
        N = reduce((lambda x, y: x*y), filtered_busses.keys())
        grid = []
        for bus, remainder in filtered_busses.items():
            n = N//bus
            inverse = pow(n,-1,bus)
            grid.append(remainder * n * inverse)

        # print(grid)
        print((sum(grid) % N) - (len(bus_ids) - 1))

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')


    return int(data[0]), data[1].split(',')


expected_time,bus_ids = get_input()
# expected_time,bus_ids = get_input('test-input.txt')

part1(expected_time, bus_ids)
# bus_ids = [1789,37,47,1889]
# part2(bus_ids)
part2_chinese_remainder(bus_ids)