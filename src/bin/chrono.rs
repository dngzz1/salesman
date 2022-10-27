#![allow(dead_code)]
use chrono::prelude::*;

fn main() {
    time_cluster_duration();
}

fn time_cluster_duration() {
    let arr = [20, 40, 80, 160, 320, 640];
    for num_points in arr {
        // let salesmen_capacities = [num_points / 2, num_points / 2];
        let salesmen_capacities = [num_points / 4; 4];
        print_cluster_duration(num_points, &salesmen_capacities);
    }
}

fn time_salesman_duration() {
    let arr = [10, 20, 40, 80, 160, 320, 640, 1280];
    for num_points in arr {
        print_salesman_duration(num_points);
    }
}

fn print_salesman_duration(num_points: usize) {
    println!(
        "Salesman with {} points: {} microseconds.",
        num_points,
        salesman_duration(num_points).num_microseconds().unwrap()
    );
}

fn salesman_duration(num_points: usize) -> chrono::Duration {
    let num_points = num_points;
    let start_time = Utc::now().time();
    let seed = 42;
    let rand_points = salesman::example::rand_points_from_chacha(num_points, seed);
    salesman::anneal::shortest_path(&rand_points, 1);
    let end_time = Utc::now().time();
    end_time - start_time
}

fn cluster_duration(num_points: usize, salesmen_capacities: &[usize]) -> chrono::Duration {
    let num_points = num_points;
    let start_time = Utc::now().time();
    let seed = 42;
    let rand_points = salesman::example::rand_points_from_chacha(num_points, seed);
    salesman::cluster::cluster_order(&rand_points, salesmen_capacities);
    let end_time = Utc::now().time();
    end_time - start_time
}

fn print_cluster_duration(num_points: usize, salesmen_capacities: &[usize]) {
    println!(
        "Salesman with {} points: {} microseconds.",
        num_points,
        cluster_duration(num_points, salesmen_capacities)
            .num_microseconds()
            .unwrap()
    );
}
