import matplotlib.pyplot as plt

x = []
y = []

input_file = open('outputs/log_temperature', 'r')
for line in input_file.readlines():
    line = line.split()
    x.append(int(line[0]))
    y.append(float(line[1]))

plt.plot(x,y,label='temperature curve')

plt.xlabel('iterations')
plt.ylabel('temperature')

plt.title('Decreasing temperature')
plt.legend()

plt.show()