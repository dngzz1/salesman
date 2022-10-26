use chrono::prelude::*;

fn main() {
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
    let rand_points = salesman::example::rand_points_from_chacha(num_points);
    salesman::anneal::shortest_path(&rand_points, 1);
    let end_time = Utc::now().time();
    end_time - start_time
}
