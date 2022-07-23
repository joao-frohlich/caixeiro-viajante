import matplotlib.pyplot as plt
import sys

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

if len(sys.argv) > 1:
    instance = sys.argv[1]
    cooling_idx = sys.argv[2]
    run_idx = sys.argv[3]
    plt.savefig('results/runs_'+instance+'/cooling_schedule_'+cooling_idx+'/'+run_idx+'.png')
else:
    plt.show()