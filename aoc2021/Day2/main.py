def format_input():
	input = []
	with open("2021/Day2/input.txt", "r") as file:
		lines = file.readlines()
	for line in lines:
		path = line.split()
		input.append([path[0], int(path[1])])
	return input

def part_1():
	input = format_input()
	x = 0
	y = 0
	for path in input:
		if path[0] == "forward":
			x += path[1]
		elif path[0] == "up":
			y -= path[1]
		elif path[0] == "down":
			y += path[1]
	return x * y
print(f"Part 1 answer: {part_1()}")

def part_2():
	input = format_input()
	x = 0
	y = 0
	aim = 0
	for path in input:
		if path[0] == "forward":
			y += aim * path[1]
			x += path[1]
		elif path[0] == "up":
			aim -= path[1]
		elif path[0] == "down":
			aim += path[1]
	return x * y

print(f"Part 2 answer: {part_2()}")