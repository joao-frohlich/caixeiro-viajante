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

fn f0(t0: f64, _ti: f64, i: usize, tn: f64, n: usize, _alpha: f64) -> f64 {
    t0-i as f64*((t0-tn)/n as f64)
}

fn f1(t0: f64, _ti: f64, i: usize, tn: f64, n: usize, _alpha: f64) -> f64 {
    (t0-tn)/(1.0+f64::powf(std::f64::consts::E,0.3*(i as f64-(n as f64)/2.0)))+tn
}

fn f2 (t0: f64, _ti: f64, i: usize, tn: f64, n: usize, _alpha: f64) -> f64 {
    let a = f64::ln(t0-tn)/f64::ln(n as f64);
    t0-f64::powf(i as f64, a)
}

fn f3 (t0: f64, _ti: f64, i: usize, tn: f64, n: usize, _alpha: f64) -> f64 {
    let i = i as f64;
    let n = n as f64;
    ((t0-tn)/2.0)*(1.0-f64::tanh(10.0*i/n - 5.0))+tn
}

fn sa(
    alpha: f64,
    samax: usize,
    max_iter: usize,
    t0: f64,
    min_t: f64,
    s: &mut Vec<usize>,
    matrix: &Vec<Vec<f64>>,
    sz: usize,
    f: fn(f64, f64, usize, f64, usize, f64) -> f64,
) -> Vec<usize> {
    let mut sr = Vec::<usize>::new();
    let mut sn = Vec::<usize>::new();
    sr.resize(sz, 0);
    sn.resize(sz, 0);
    sr.copy_from_slice(s);
    let mut iter_t = 0;
    let mut t = t0;
    let mut iter = 0;
    let mut log_temperature_file = fs::File::create("outputs/log_temperature").unwrap();
    let mut log_solution_file = fs::File::create("outputs/log_solution").unwrap();
    while iter < max_iter {
        writeln!(log_temperature_file, "{} {}", iter, t).unwrap();
        while iter_t < samax {
            iter_t += 1;
            sn.copy_from_slice(s);
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
            let delta = get_cost(&sr, matrix, sz) - get_cost(s, matrix, sz);
            if delta < 0.0 {
                s.copy_from_slice(&sn);
                if get_cost(&sn, matrix, sz) < get_cost(&sr, matrix, sz) {
                    sr.copy_from_slice(&sn);
                }
            } else {
                let between = Uniform::<f64>::from(0.0..=1.0);
                let mut rng = rand::thread_rng();
                let x = between.sample(&mut rng);
                if x < f64::powf(std::f64::consts::E, -delta / t) {
                    s.copy_from_slice(&sn);
                }
            }
        }
        writeln!(log_solution_file, "{} {}", iter, get_cost(&s, matrix, sz)).unwrap();
        if t > min_t {
            t = f(t0, t, iter, min_t, max_iter, alpha);
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
    // solution = sa(0.9, 5, 1000, 1000.0, 0.0001, &mut solution, &matrix, sz, f0);
    // solution = sa(0.9, 5, 100, 100000.0, 0.0001, &mut solution, &matrix, sz, f1);
    // solution = sa(0.9, 5, 1000, 100000.0, 5.0001, &mut solution, &matrix, sz, f2);
    solution = sa(0.9, 1, 100000, 1500.0, 2.0001, &mut solution, &matrix, sz, f3);
    let new_cost = get_cost(&solution, &matrix, sz);
    println!("Some SA Solution: {}", new_cost);
    // println!("Optmization: {}", new_cost/cost);
}
