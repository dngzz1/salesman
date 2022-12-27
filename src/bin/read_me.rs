#![allow(dead_code)]
fn main() {
    // one();
    // two();
    // three_closed();
    // three_open();
    five();
    // ten();
}

fn one() {
    let num_points = 20;
    let seed = None;
    let intensity = 10.0;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let salesmen_capacities = [20; 1];

    let is_loop = true;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let stringing = salesman::string::get_stringing(&points, &salesmen_capacities, &order, is_loop);
    let svg = salesman::utils::svg::create_svg(&stringing);
    svg::save("images/read_me/one.svg", &svg).unwrap();
}

fn two() {
    let num_points = 40;
    let seed = None;
    let intensity = 10.0;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let salesmen_capacities = [20; 2];

    let is_loop = true;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let stringing = salesman::string::get_stringing(&points, &salesmen_capacities, &order, is_loop);
    let svg = salesman::utils::svg::create_svg(&stringing);
    svg::save("images/read_me/two.svg", &svg).unwrap();
}

fn three_closed() {
    let num_points = 40;
    let seed = Some(42);
    let intensity = 10.0;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let salesmen_capacities = [20, 10, 10];

    let is_loop = true;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let stringing = salesman::string::get_stringing(&points, &salesmen_capacities, &order, is_loop);
    let svg = salesman::utils::svg::create_svg(&stringing);
    svg::save("images/read_me/three_closed.svg", &svg).unwrap();
}

fn three_open() {
    let num_points = 40;
    let seed = Some(42);
    let intensity = 10.0;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let salesmen_capacities = [20, 10, 10];

    let is_loop = false;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let stringing = salesman::string::get_stringing(&points, &salesmen_capacities, &order, is_loop);
    let svg = salesman::utils::svg::create_svg(&stringing);
    svg::save("images/read_me/three_open.svg", &svg).unwrap();
}

fn five() {
    let num_points = 40;
    let seed = Some(42);
    let intensity = 10.0;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let salesmen_capacities = [8; 5];

    let is_loop = true;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let stringing = salesman::string::get_stringing(&points, &salesmen_capacities, &order, is_loop);
    let svg = salesman::utils::svg::create_svg(&stringing);
    svg::save("images/read_me/five.svg", &svg).unwrap();
}

fn ten() {
    let num_points = 100;
    let seed = Some(42);
    let intensity = 10.0;
    let points = salesman::utils::rand::get_points(num_points, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let salesmen_capacities = [10; 10];

    let is_loop = false;
    let order = salesman::string::get_string_order(
        &points,
        &salesmen_capacities,
        &distance_fn,
        is_loop,
        intensity,
        seed,
    );
    let stringing = salesman::string::get_stringing(&points, &salesmen_capacities, &order, is_loop);
    let svg = salesman::utils::svg::create_svg(&stringing);
    svg::save("images/read_me/ten.svg", &svg).unwrap();
}
