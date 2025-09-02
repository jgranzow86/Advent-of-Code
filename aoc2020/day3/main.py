import pathlib
from tqdm import tqdm

def main(input):
	with open(input, mode='r') as opened_file:
		data = opened_file.read()
	
	map = data.split(sep='\n')

	hits_on_paths = []

	hits_on_paths.append(check_path(1,1, map))
	hits_on_paths.append(check_path(3,1, map))
	hits_on_paths.append(check_path(5,1, map))
	hits_on_paths.append(check_path(7,1, map))
	hits_on_paths.append(check_path(1,2, map))

	answer = 1

	for x in hits_on_paths:
		answer *= x
	print(answer)

def check_path(right, down, map):
	pos = 0
	tree_hit = 0

	for y in range(len(map)):
		if not (y % down):
			x = wrap(pos, len(map[y]))
			if map[y][x] == "#":
				tree_hit += 1
		pos += right
			


	# for row in map:
	# 	wrapped_pos = wrap(pos, len(row))
	# 	in_space = row[wrapped_pos]
	# 	if in_space == '#':
	# 		tree_hit += 1
	# 	pos += run

	return tree_hit

def wrap(pos, max):
	if pos < max:
		return pos
	else:
		return pos % max

if __name__ == "__main__":
	main(pathlib.Path(__file__).parent.joinpath("input.txt"))
