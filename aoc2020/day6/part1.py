import pathlib

class customsGroup():
	def __init__(self, data):
		self.answers = data.split(sep="\n")

	def count(self):
		yesAnswers = []
		for person in self.answers:
			for answer in person:
				if answer not in yesAnswers:
					yesAnswers.append(answer)
		return len(yesAnswers)

def main(input):
	with open(input, mode='r') as opened_file:
		data = opened_file.read()

	groups = []
	for _ in data.split(sep="\n\n"):
		groups.append(customsGroup(_))
	
	total = 0
	for _ in groups:
		total += _.count()

	print(f"Sum: {total}")

if __name__ == "__main__":
	main(pathlib.Path(__file__).parent.joinpath("input.txt"))
