#!/usr/bin/python3.8

from copy import copy

def part1(puzzle_input):
    player_1 = puzzle_input[0]
    player_2 = puzzle_input[1]
    round_count = 0
    while len(player_1) > 0 and len(player_2) > 0:
        p1_card = player_1.pop(0)
        p2_card = player_2.pop(0)

        if p1_card > p2_card:
            player_1 += [p1_card,p2_card]
        else:
            player_2 += [p2_card,p1_card]
        round_count +=1

    # print(puzzle_input)
    winning_player = player_1 if len(player_2) == 0 else player_2
    print_score(winning_player)

def print_score(winning_player):
    winning_score = 0
    for index, card in enumerate(winning_player):
        winning_score += card * (len(winning_player) - index)

    print(winning_score)

def part2(puzzle_input):
    player_1 = puzzle_input[0]
    player_2 = puzzle_input[1]

    # print(puzzle_input)

    winning_player = play_game(player_1, player_2)
    print_score(puzzle_input[winning_player - 1])

master_game_state = set()

def play_game(player_1,player_2):
    game_history = set()
    winning_player = None
    while not winning_player:
        game_state = (tuple(player_1), tuple(player_2))
        if game_state in game_history:
            winning_player = 1
            break
        game_history.add(game_state)

        p1_card = player_1.pop(0)
        p2_card = player_2.pop(0)

        round_winner = play_round(p1_card, p2_card, len(player_1), len(player_2))
        if round_winner == -1:
            # print(game_count, player_1,player_2)
            round_winner = play_game(copy(player_1[:p1_card]), copy(player_2[:p2_card]))

        if round_winner == 1:
            player_1 += [p1_card,p2_card]
        elif round_winner == 2:
            player_2 += [p2_card,p1_card]
        else:
            print('somets gone wrong')

        if len(player_1) == 0:
            winning_player = 2
            break
        if len(player_2) == 0:
            winning_player = 1
            break
        

    return winning_player

def play_round(p1_card, p2_card, p1_count, p2_count):
    if p1_count >= p1_card and p2_count >= p2_card:
        return -1
    else:
        if p1_card > p2_card:
            return 1
        else:
            return 2

def get_input(input_file_name = "input.txt"):
    with open(input_file_name, 'r') as file:
        data = file.read().split('\n\n')

    
    puzzle_input = [list(map(int,player.split('\n')[1:])) for player in data]

    return puzzle_input 


puzzle_input = get_input()
test_input = get_input('test-input.txt')

# part1(puzzle_input)
part1(test_input)

puzzle_input = get_input()
test_input = get_input('test-input.txt')

part2(puzzle_input)
# part2(test_input)
