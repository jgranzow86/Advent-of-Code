import pathlib
import re
from tqdm import tqdm

class Passport():
	def __init__(self, raw_data):
		data = re.split('\n| ', raw_data)
		
		self.birth_year = None
		self.issue_year = None
		self.expiration_year = None
		self.height = None
		self.hair_color = None
		self.eye_color = None
		self.passport_id = None
		self.country_id = None
		
		for each in data:
			field = each.split(sep=":")[0]
			value = each.split(sep=":")[1]

			if field == "byr":
				value = int(value)
				if value >= 1920 and value <= 2002:
					self.birth_year = value

			elif field == "iyr":
				value = int(value)
				if value >= 2010 and value <= 2020:
					self.issue_year = value

			elif field == "eyr":
				value = int(value)
				if value >= 2020 and value <= 2030:
					self.expiration_year = value

			elif field == "hgt":
				if re.match(r'^\d{3,}(cm)', value):
					height = int(value.split(sep="cm")[0])
					if height >= 150 and height <= 193:
						self.height = value
				elif re.match(r'^\d{2,}(in)', value):
					height = int(value.split(sep="in")[0])
					if height >= 59 and height <= 76:
						self.height = value

			elif field == "hcl":
				if re.fullmatch(r'\#[\d|a-f]{6}', value):
					self.hair_color = value

			elif field == "ecl":
				if re.match(r'^amb|blu|brn|gry|grn|hzl|oth$', value):
					self.eye_color = value

			elif field == "pid":
				if re.match(r'^\d{9}$', value):
					self.passport_id = value

			elif field == "cid":
				self.country_id = value

	@property
	def isValid(self):
		if (
			self.birth_year != None and 
			self.issue_year != None and
			self.expiration_year != None and
			self.height != None and
			self.hair_color != None and
			self.eye_color != None and
			self.passport_id != None
			):
			return True
		else:
			return False


def main(input):
	with open(input, mode='r') as opened_file:
		data = opened_file.read()
	
	pasports = []
	for each in data.split(sep="\n\n"):
		pasports.append(Passport(each))

	valid_passports = 0
	invalid_passports = 0

	for pasport in pasports:
		if pasport.isValid:
			valid_passports += 1
		else:
			invalid_passports += 1

	print(f"Valid passports: {valid_passports}\nInvalid Passports: {invalid_passports}")

if __name__ == "__main__":
	main(pathlib.Path(__file__).parent.joinpath("input.txt"))
