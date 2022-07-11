import matplotlib.pyplot as plt

x = []
y = []

input_file = open('outputs/log_solution', 'r')
for line in input_file.readlines():
    line = line.split()
    x.append(int(line[0]))
    y.append(float(line[1]))

plt.plot(x,y,label='solution cost')

plt.xlabel('iterations')
plt.ylabel('solution')

plt.title('Convergence solution')
plt.legend()

plt.show()