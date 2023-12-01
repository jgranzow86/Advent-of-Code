#!/usr/bin/python

def format_input():
	input = []
	with open("2021/Day1/input.txt", 'r') as file:
		lines = file.readlines()
	for line in lines:
		input.append(int(line.replace('\n', '')))
	return input

def part_1():
	"""
	Get the number of times the depth increases
	"""
	input = format_input()
	count = -1
	previous_depth = 0
	for current_depth in input:
		current_depth = int(current_depth)
		if previous_depth < current_depth:
			count += 1
		previous_depth = current_depth
	return count

def part_2():
	input = format_input()
	count = -1
	previous_sum = 0
	for index in range(1, len(input) - 1):
		sum = input[index - 1] + input[index] + input[index + 1]
		if previous_sum < sum:
			count += 1
		previous_sum = sum
	return count

print(f"Part 1 answer: {part_1()}")
print(f"Part 2 answer: {part_2()}")