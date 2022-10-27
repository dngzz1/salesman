#![allow(dead_code)]
use chrono::prelude::*;

fn main() {
    time_cluster_duration_varying_intensity();
}

fn time_cluster_duration_varying_num_points() {
    let arr = [20, 40, 80, 160, 320, 640];
    let intensity = 10.0;
    let seed = Some(42);
    for num_points in arr {
        let rand_points = salesman::example::rand_points_from_chacha(num_points, seed);
        let salesmen_capacities = [num_points / 4; 4];
        let f = || {
            salesman::cluster::best_cluster(&rand_points, &salesmen_capacities, intensity, seed);
        };
        println!(
            "Clustering with {} points: {} milliseconds.",
            num_points,
            get_duration(f).num_milliseconds()
        );
    }
}

fn time_cluster_duration_varying_intensity() {
    let arr = [10., 10.5, 11., 11.5, 12.];
    let num_points = 40;
    let seed = Some(42);
    let rand_points = salesman::example::rand_points_from_chacha(num_points, seed);
    for intensity in arr {
        let salesmen_capacities = [num_points / 4; 4];
        let f = || {
            salesman::cluster::best_cluster(&rand_points, &salesmen_capacities, intensity, seed);
        };
        println!(
            "Clustering with intensity {}: {} milliseconds.",
            intensity,
            get_duration(f).num_milliseconds()
        );
    }
}

fn time_salesman_duration() {
    println!("Timing Traveling Salesman by Simulated Annealing...");
    let arr = [10, 20, 40, 80, 160, 300, 320];
    let seed = Some(42);
    for num_points in arr {
        let rand_points = salesman::example::rand_points_from_chacha(num_points, seed);
        let f = || {
            salesman::anneal::shortest_path(&rand_points, 1, true, seed);
        };
        println!(
            "Salesman with {} points: {} milliseconds.",
            num_points,
            get_duration(f).num_milliseconds()
        );
    }
}

fn get_duration<F>(f: F) -> chrono::Duration
where
    F: FnOnce(),
{
    let start_time = Utc::now().time();
    f();
    let end_time = Utc::now().time();
    end_time - start_time
}
