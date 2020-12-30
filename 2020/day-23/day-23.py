#!/usr/bin/python3.8
class Node:
    def __init__(self, dataval=None):
        self.dataval = dataval
        self.nextval = None

    def __str__(self):
        return f'Me: {self.dataval}, Next:{self.nextval.dataval}'

class SLinkedList:
    def __init__(self):
        self.headval = None
        self.map = {}
        self.length = 0

    def __str__(self):
        printval = self.headval
        print_list = []
        for _ in range(self.length):
        # while printval is not None:
            print_list.append(str(printval.dataval))
            # print(printval, printval.nextval)
            # print (printval.dataval)
            printval = printval.nextval

        return ','.join(print_list)
        # print(print_list)
    
    def create_map(self):
        currentval = self.headval
        for _ in range(self.length):
            self.map[currentval.dataval] = currentval
            currentval = currentval.nextval


def create_linked_from_list(list_to_link):
    linked = SLinkedList()
    linked.headval = Node(list_to_link.pop(0))
    linked.length = 1
    currentNode = linked.headval
    for val in list_to_link:
        nextNode = Node(val)
        linked.length += 1
        currentNode.nextval = nextNode
        currentNode = nextNode

    currentNode.nextval = linked.headval
    return linked

def part1(puzzle_input):
    cups = list(puzzle_input[0])
    cups = list(map(int,cups))
    current_cup = cups[0]

    linked_cups = create_linked_from_list(cups)
    # linked_cups.listprint()
    # print(linked_cups)
    linked_cups.create_map()
    # print(linked_cups.map)
    # print(linked_cups.map[3].nextval.dataval)

    # print(cups)
    for _ in range(100):
        current_cup = move_cups_using_linked_list(linked_cups, current_cup)
        # print(linked_cups)
        # pass
        # current_cup = move_cups(cups,current_cup)
        # print(cups, current_cup)
    # print(cups, current_cup)
    print(get_answer_from_linked(linked_cups))

def get_answer_string(cups):
    index_of_one = cups.index(1)
    answer_string = ''
    next_label = (index_of_one + 1) % len(cups)
    while len(answer_string) < len(cups) - 1:
        answer_string += str(cups[next_label])
        next_label = (next_label + 1) % len(cups)

    return answer_string

def get_answer_from_linked(linked_cups):
    printval = linked_cups.map[1].nextval
    print_list = []
    for _ in range(linked_cups.length - 1):
        print_list.append(str(printval.dataval))
        printval = printval.nextval

    return ''.join(print_list)

def move_cups_using_linked_list(linked_cups, current_cup):
    current_cup_node = linked_cups.map[current_cup]
    first_removed = current_cup_node.nextval
    second_removed = current_cup_node.nextval.nextval
    third_removed = current_cup_node.nextval.nextval.nextval  
    get_new_next = current_cup_node.nextval.nextval.nextval.nextval

    removed_cups = [first_removed.dataval, second_removed.dataval, third_removed.dataval]

    destination_cup = current_cup - 1
    if destination_cup == 0:
            destination_cup = linked_cups.length
    while destination_cup in removed_cups:
        destination_cup -= 1
        if destination_cup == 0:
            destination_cup = linked_cups.length

    # print(current_cup, destination_cup, removed_cups)
        
    destination_cup_node = linked_cups.map[destination_cup]
    current_cup_node.nextval = get_new_next
    temp = destination_cup_node.nextval
    # print(destination_cup_node,temp,first_removed, third_removed)
    destination_cup_node.nextval = first_removed
    # print(destination_cup_node, destination_cup_node.nextval)
    third_removed.nextval = temp
    # print(destination_cup_node, destination_cup_node.nextval)

    # print(linked_cups)
    return current_cup_node.nextval.dataval

def move_cups(cups, current_cup):
    current_cup_index = cups.index(current_cup)
    removed_cups = [cups.pop((cups.index(current_cup) + 1) % len(cups)) for _ in range(1, 4)]


    destination_cup = current_cup - 1
    while destination_cup not in cups:
        if destination_cup == 0:
            destination_cup = len(cups) + 4
        destination_cup -= 1

    destination_cup_index = cups.index(destination_cup)

    cups[destination_cup_index+1:destination_cup_index+1] = removed_cups
    next_cup_index = (cups.index(current_cup) + 1) % len(cups)
    return cups[next_cup_index]



def part2(puzzle_input):
    cups = list(puzzle_input[0])
    cups = list(map(int,cups))
    current_cup = cups[0]
    ONE_MILION = 1000000
    cups.extend(range(len(cups)+1,ONE_MILION + 1))

    current_cup = cups[0]
    linked_cups = create_linked_from_list(cups)
    linked_cups.create_map()
  

    # print(cups)
    for _ in range(ONE_MILION * 10):
        # print(_)
        current_cup = move_cups_using_linked_list(linked_cups, current_cup)
    
    print(linked_cups.map[1].nextval.dataval * linked_cups.map[1].nextval.nextval.dataval)



def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n')

    return data


puzzle_input = get_input()
test_input = get_input('test-input.txt')

# part1(puzzle_input)
part1(test_input)
part2(puzzle_input)
# part2(test_input)