use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;

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
    println!("{}", cost);
}
