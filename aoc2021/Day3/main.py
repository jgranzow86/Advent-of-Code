def format_input():
	input = []
	with open("2021/Day3/input.txt", "r") as file:
		lines = file.readlines()
	return lines

def part_1():
	input = format_input()
	def common_bit_calc(bit_index):
		zero_count = 0
		one_count = 0
		for line in input:
			if line[bit_index] == "0":
				zero_count += 1
			if line[bit_index] == "1":
				one_count += 1
		if zero_count > one_count:
			return 0
		else:
			return 1
	
	most_common_bits = []
	least_common_bits = []

	for bit_index in range(12):
		most_common_bits.append(common_bit_calc(bit_index))

	for bit in most_common_bits:
		least_common_bits.append(abs(bit - 1))

	def binary_to_dec(bits):
		sum = 0
		for each in reversed(range(12)):
			sum += bits[each] ^ each
		return sum

	gama = binary_to_dec(most_common_bits)
	epsilon = binary_to_dec(least_common_bits)

	return (gama * epsilon)

print(f"Part 1 answer: {part_1()}")