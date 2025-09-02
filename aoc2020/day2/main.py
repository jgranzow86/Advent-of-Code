import pathlib
from tqdm import tqdm

def main(input):
	with open(input, mode='r') as opened_file:
		raw_data = opened_file.read()

	line_data = raw_data.split(sep="\n")
	Invalid_count = 0

	for line in tqdm(line_data, desc="Checking Passwords"):
		min_max = line.split()[0]
		char = line.split()[1][0]
		password = line.split()[2]

		min = int(min_max.split(sep='-')[0])
		max = int(min_max.split(sep='-')[1])
	
		# if validate_password(min, max, char, password):
		# 	Invalid_count += 1

		if part2_validation(min, max, char, password):
			Invalid_count += 1
	
	print(f'Total valid password: {Invalid_count}')

def part2_validation(pos1, pos2, letter, password):
	if (password[pos1-1] == letter) != (password[pos2-1] == letter):
		return True

def validate_password(min, max, letter, password):
	letter_count = 0

	for char in password:
		if char == letter:
			letter_count += 1
	
	if letter_count >= min and letter_count <= max:
		return True
	else:
		return False

if __name__ == "__main__":
	main(pathlib.Path(__file__).parent.joinpath("input.txt"))
