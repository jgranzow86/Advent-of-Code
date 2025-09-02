import pathlib

print(pathlib.Path(__file__).parent)

def main():
	input_file_name = "input.txt"
	input_file = str(pathlib.Path(__file__).parent.joinpath(input_file_name))
	
	with open(input_file, mode="r") as openedfile:
		datastr = str(openedfile.read()).split()
		data = []
		for x in datastr:
			data.append(int(x))

	found = False

	for x in data:
		for y in data:
			if (x + y) == 2020:
				print (f'Part 1: {x*y}')
				found = True
				break
		if found:
			break

	# Part 2
	found = False

	for x in data:
		for y in data:
			for z in data:
				if (x + y + z) == 2020:
					print (f'Part 2: {x*y*z}')
					found = True
					break
			if found:
				break
		if found:
			break

if __name__ == "__main__":
	main()
