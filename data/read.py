import re
with open("data.txt",mode='r') as file:
	for line in file:
		res = re.findall('{[^}]+}', line, re.S)
		print(res)
                