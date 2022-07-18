#!/bin/bash

for i in {1..10}
do
    cargo run --release
    # python3 generate_solution_cost_graphics.py
    # python3 generate_temperature_graphics.py
done