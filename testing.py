import os
import matplotlib.pyplot as plt

run = "cargo run --release "
parameter = "\"otable\" 5 "

x = []  # length
y = []  # millis

for i in range(100, 51666000, 100):
    res = os.popen(run + parameter + i).read()
    y.append(int(res))
    x.append(i)

plt.xlabel("Skips")
plt.ylabel("Milliseconds")
plt.plot(x, y)
plt.show()

""" Eventuelt gennemsnitstider
for i in range(1, 14):
	sum = 0
	for i in range(0, 7):
    	res = os.popen(run).read()
    	sum += int(res)
    y.append(sum / 7)
    x.append(i)
"""