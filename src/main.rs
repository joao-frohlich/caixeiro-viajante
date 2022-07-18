use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::io::Write;

fn read_points(instance: usize) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
    let path = "inputs/".to_owned() + &instance.to_string();
    let contents = fs::read_to_string(path).expect("Something went wrong");
    for line in contents.split('\n') {
        let values: Vec<&str> = line.split_whitespace().collect();
        let x = values[1].parse::<i32>().unwrap();
        let y = values[2].parse::<i32>().unwrap();
        points.push((x, y));
    }
    points
}

fn euclidean_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> f64 {
    let x_dist = i32::pow(x1 - x2, 2);
    let y_dist = i32::pow(y1 - y2, 2);
    ((x_dist + y_dist) as f64).sqrt()
}

fn get_matrix(points: &Vec<(i32, i32)>, sz: usize) -> Vec<Vec<f64>> {
    let mut ret = Vec::<Vec<f64>>::new();
    for i in 0..sz {
        ret.push(Vec::<f64>::new());
        for j in 0..sz {
            ret[i].push(euclidean_distance(points[i], points[j]));
        }
    }
    ret
}

fn get_cost(solution: &Vec<usize>, matrix: &Vec<Vec<f64>>, sz: usize) -> f64 {
    let mut cost: f64 = matrix[solution[0]][solution[sz - 1]];
    for i in 1..sz {
        cost += matrix[solution[(i as i32 - 1) as usize]][solution[i]];
    }
    cost
}

fn cooling_schedule_0 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    t0-i*((t0-tn)/n)
}

fn cooling_schedule_1 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    t0*f64::powf(tn/t0,i/n)
}

fn cooling_schedule_2 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    let a = ((t0-tn)*(n+1.0))/n;
    let b = t0-a;
    (a/(i+1.0))+b
}

fn cooling_schedule_3 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    let a = f64::ln(t0-tn)/f64::ln(n);
    t0-f64::powf(i, a)
}

fn cooling_schedule_4 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    ((t0-tn)/(1.0+f64::powf(std::f64::consts::E,0.3*(i-n/2.0))))+tn
}

fn cooling_schedule_5 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    ((t0-tn)/2.0)*(1.0+f64::cos((i*std::f64::consts::PI)/n))+tn
}

fn cooling_schedule_6 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    ((t0 - tn) / 2.0) * (1.0 - f64::tanh(10.0 * i / n - 5.0)) + tn
}

fn cooling_schedule_7 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    (t0-tn)/f64::cosh(10.0*i/n)+tn
}

fn cooling_schedule_8 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    let a = (1.0/n)*f64::ln(t0/tn);
    t0*f64::powf(std::f64::consts::E, -a*i*i)
}

fn cooling_schedule_9 (t0: f64, i: usize, tn: f64, n: usize) -> f64 {
    let i = i as f64;
    let n = n as f64;
    let a = (1.0/(n*n))*f64::ln(t0/tn);
    t0*f64::powf(std::f64::consts::E, -a*i*i)
}

fn sa(
    samax: usize,
    max_iter: usize,
    t0: f64,
    min_t: f64,
    s: &mut Vec<usize>,
    matrix: &Vec<Vec<f64>>,
    sz: usize,
    f: fn(f64, usize, f64, usize) -> f64,
) -> Vec<usize> {
    let mut sr = Vec::<usize>::new();
    let mut sn = Vec::<usize>::new();
    sr.resize(sz, 0);
    sn.resize(sz, 0);
    sr.copy_from_slice(s);
    let mut iter_t = 0;
    let mut t = t0;
    let mut iter = 0;
    // let mut log_temperature_file = fs::File::create("outputs/log_temperature").unwrap();
    // let mut log_solution_file = fs::File::create("outputs/log_solution").unwrap();
    // let mut log_arrays_file = fs::File::create("outputs/log_array").unwrap(); 
    while iter < max_iter {
        // writeln!(log_temperature_file, "{} {}", iter, t).unwrap();
        while iter_t < samax {
            iter_t += 1;
            sn.copy_from_slice(s);
            // writeln!(log_arrays_file, "\n\nIter: {}, {}", iter, iter_t).unwrap();
            // writeln!(log_arrays_file, "s: {:?}", s).unwrap();
            // writeln!(log_arrays_file, "sn after copy: {:?}", sn).unwrap();
            let between = Uniform::from(1..=5);
            let mut rng = rand::thread_rng();
            let num_changes = between.sample(&mut rng);
            let mut cont = 0;
            while cont < num_changes {
                let between = Uniform::from(0..sz - 1);
                let mut rng = rand::thread_rng();

                let x = between.sample(&mut rng);
                let mut y = between.sample(&mut rng);
                while x == y {
                    y = between.sample(&mut rng);
                }
                sn.swap(x, y);
                cont += 1;
            }
            let delta = get_cost(&sn, matrix, sz) - get_cost(s, matrix, sz);
            if delta < 0.0 {
                s.copy_from_slice(&sn);
                if get_cost(&sn, matrix, sz) < get_cost(&sr, matrix, sz) {
                    sr.copy_from_slice(&sn);
                }
            } else {
                let between = Uniform::<f64>::from(0.0..=1.0);
                let mut rng = rand::thread_rng();
                let x = between.sample(&mut rng);
                if f64::powf(std::f64::consts::E, -delta / t) > 1.0 {
                    println!("Deu ruim");
                }
                if x < f64::powf(std::f64::consts::E, -delta / t) {
                    s.copy_from_slice(&sn);
                }
            }
        }
        // writeln!(log_solution_file, "{} {}", iter, get_cost(&s, matrix, sz)).unwrap();
        if t > min_t {
            t = f(t0, iter, min_t, max_iter);
        }
        iter_t = 0;
        iter += 1;
    }
    sr
}

fn main() {
    let sz = 51;
    let points = read_points(sz);
    let matrix = get_matrix(&points, sz);
    let mut solution = Vec::<usize>::new();
    for i in 0..sz {
        solution.push(i);
    }
    let mut rng = thread_rng();
    solution.shuffle(&mut rng);
    let cost = get_cost(&solution, &matrix, sz);
    println!("Initial Solution: {}", cost);

    let functions = [cooling_schedule_0, cooling_schedule_1, cooling_schedule_2, cooling_schedule_3, cooling_schedule_4, cooling_schedule_5, cooling_schedule_6, cooling_schedule_7, cooling_schedule_8, cooling_schedule_9];

    let mut cont = 0;
    for function in functions {
        println!("Results for cooling schedule {}", cont);
        let mut samax = 5;
        while samax <= 30 {
            println!("\tmetropolis: {}", samax);
            let mut max_iter = 5000;
            while max_iter <= 100000 {
                println!("\t\tmax_iter: {}", max_iter);
                let mut temp = 100.0;
                while temp < 5005.0 {
                    println!("\t\t\tinitial temperature: {}", temp);
                    let mut new_solution = solution.clone();
                    new_solution = sa(samax, max_iter, temp, 0.0, &mut new_solution, &matrix, sz, function);
                    println!("\t\t\t\tcost: {}", get_cost(&new_solution, &matrix, sz));
                    temp += 100.0;
                }
                max_iter += 5000;
            }
            samax += 5;
        }
        cont += 1;
    }

    // let mut partial_solution_0 = solution.clone();
    // partial_solution_0 = sa(5, 50000, 100.0, 0.0001, &mut partial_solution_0, &matrix, sz, cooling_schedule_0);
    // let partial_cost_0 = get_cost(&partial_solution_0, &matrix, sz);
    // println!("Solution for cooling schedule 0: {}", partial_cost_0);
    // let mut partial_solution_1 = solution.clone();
    // partial_solution_1 = sa(5, 50000, 100.0, 0.0001, &mut partial_solution_1, &matrix, sz, cooling_schedule_1);
    // let partial_cost_1 = get_cost(&partial_solution_1, &matrix, sz);
    // println!("Solution for cooling schedule 1: {}", partial_cost_1);
    // let mut partial_solution_2 = solution.clone();
    // partial_solution_2 = sa(5, 50000, 100.0, 0.0001, &mut partial_solution_2, &matrix, sz, cooling_schedule_2);
    // let partial_cost_2 = get_cost(&partial_solution_2, &matrix, sz);
    // println!("Solution for cooling schedule 2: {}", partial_cost_2);
    // let mut partial_solution_3 = solution.clone();
    // partial_solution_3 = sa(5, 50000, 100.0, 0.0001, &mut partial_solution_3, &matrix, sz, cooling_schedule_3);
    // let partial_cost_3 = get_cost(&partial_solution_3, &matrix, sz);
    // println!("Solution for cooling schedule 3: {}", partial_cost_3);
    // let mut partial_solution_4 = solution.clone();
    // partial_solution_4 = sa(5, 50000, 100.0, 0.0001, &mut partial_solution_4, &matrix, sz, cooling_schedule_4);
    // let partial_cost_4 = get_cost(&partial_solution_4, &matrix, sz);
    // println!("Solution for cooling schedule 4: {}", partial_cost_4);
    // let mut partial_solution_5 = solution.clone();
    // partial_solution_5 = sa(5, 50000, 100.0, 0.0001, &mut partial_solution_5, &matrix, sz, cooling_schedule_5);
    // let partial_cost_5 = get_cost(&partial_solution_5, &matrix, sz);
    // println!("Solution for cooling schedule 5: {}", partial_cost_5);
    // let mut partial_solution_6 = solution.clone();
    // partial_solution_6 = sa(5, 40000, 100.0, 0.0, &mut partial_solution_6, &matrix, sz, cooling_schedule_6);
    // let partial_cost_6 = get_cost(&partial_solution_6, &matrix, sz);
    // println!("Solution for cooling schedule 6: {}", partial_cost_6);
    // solution = sa(5, 10000, 100.0, 0.0001, &mut solution, &matrix, sz, cooling_schedule_3);
    // solution = sa(5, 10000, 300.0, 0.0001, &mut solution, &matrix, sz, cooling_schedule_6);
    // let new_cost = get_cost(&solution, &matrix, sz);
    // println!("First SA Solution: {}", new_cost);
    // println!("Optmization: {}", new_cost/cost);
}
