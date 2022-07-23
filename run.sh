#!/bin/bash

# 51 cities instance:

# Cooling schedule	|	Metropolis	|	Iterations	|	Temperature	|	Cost
# --------------------+---------------+---------------+---------------+------------+
# 7					|	30			|	85_000		|	3_400		|	436.413
# --------------------+---------------+---------------+---------------+------------+
# 6					|	20			|	95_000		|	700			|	440.811
# --------------------+---------------+---------------+---------------+------------+
# 3					|	25			|	95_000		|	100			|	446.817


# Cooling schedule 3
for i in {1..10}
do
    cargo run --release 51 25 95000 100.0 3 >> results/runs_51/cooling_schedule_3/costs
    python3 generate_solution_cost_graphics.py 51 3 $i
done

for i in {1..10}
do
    cargo run --release 51 20 95000 700.0 6 >> results/runs_51/cooling_schedule_6/costs
    python3 generate_solution_cost_graphics.py 51 6 $i
done

for i in {1..10}
do
    cargo run --release 51 30 85000 3400.0 7 >> results/runs_51/cooling_schedule_7/costs
    python3 generate_solution_cost_graphics.py 51 7 $i
done

# 100 cities instance:

# Cooling schedule	|	Metropolis	|	Iterations	|	Temperature	|	Cost
# --------------------+---------------+---------------+---------------+------------+
# 3					|	25			|	400_000		|	700			|	22389.753
# --------------------+---------------+---------------+---------------+------------+
# 5					|	25			|	300_000		|	400			|	22545.668
# --------------------+---------------+---------------+---------------+------------+
# 7					|	25			|	400_000		|	4600		|	22945.323

for i in {1..10}
do
    cargo run --release 100 25 400000 700.0 3 >> results/runs_100/cooling_schedule_3/costs
    python3 generate_solution_cost_graphics.py 100 3 $i
done

for i in {1..10}
do
    cargo run --release 100 25 300000 400.0 5 >> results/runs_100/cooling_schedule_5/costs
    python3 generate_solution_cost_graphics.py 100 5 $i
done

for i in {1..10}
do
    cargo run --release 100 25 400000 4600.0 7 >> results/runs_100/cooling_schedule_7/costs
    python3 generate_solution_cost_graphics.py 100 7 $i
done