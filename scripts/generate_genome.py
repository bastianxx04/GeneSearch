import sys
import random

filename = f"resources/genomes/rand-{sys.argv[1]}.fa"
letters = ['A', 'C', 'G', 'T']
result = ""

for i in range(0, int(sys.argv[1])):
    result += random.choice(letters)

with open(filename, 'a') as f:
    print(result, file=f)
