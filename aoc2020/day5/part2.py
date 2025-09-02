import pathlib
import re
from tqdm import tqdm

def getSeatID(row, col):
	return row * 8 + col

def getRowColumn(bsp):
	fronBack, leftRight = bsp[:7], bsp[-3:]
	rows = []
	for x in range(128):
		rows.append(x)

	cols = []
	for x in range(8):
		cols.append(x)

	for each in fronBack:
		split = int(len(rows) / 2)
		if each == "F":
			rows = rows[:split]
		elif each == "B":
			split *= -1
			rows = rows[split:]

	for each in leftRight:
		split = int(len(cols) / 2)
		if each == "L":
			cols = cols[:split]
		elif each == "R":
			split *= -1
			cols = cols[split:]

	return rows[0], cols[0]

def main(input):
	with open(input, mode='r') as opened_file:
		data = opened_file.read()
	
	seatIDs = []
	for x in range(128):
		for y in range(8):
			seatIDs.append(x * 8 + y)

	bsp = data.split(sep="\n")
	for each in bsp:
		row, col = getRowColumn(each)
		seatID = getSeatID(row, col)
		seatIDs.remove(seatID)

	for _ in seatIDs:
		print(f"Possible seat ID: {_}")



	
	print("wait")

if __name__ == "__main__":
	main(pathlib.Path(__file__).parent.joinpath("input.txt"))
