import os
import matplotlib.pyplot as plt

build = "cargo build --release --quiet"
run = os.path.join("target", "release", "gene_search.exe")
parameter = "otable 5"

x = []  # length
y = []  # millis

"""
os.popen(build).read()

for i in range(100, 50000200, 10000000):
    print(f"running {str(i)}")
    res = os.popen(f"{run} {parameter} {str(i)}").read()
    y.append(int(res))
    x.append(i)
"""
path = os.path.join("results", "graphs")
os.mkdir(path)
x = [2, 5]  # length
y = [3, 4]  # millis
plt.xlabel("String length")
plt.ylabel("Milliseconds")
plt.plot(x, y)
plt.show()
figure_location = "" #os.path.join("results", "graphs")
plt.savefig(figure_location + "-".join(parameter.split()) + ".pdf")

""" Eventuelt gennemsnitstider
for i in range(1, 14):
	sum = 0
	for i in range(0, 7):
    	res = os.popen(run).read()
    	sum += int(res)
    y.append(sum / 7)
    x.append(i)
"""