// use rand::Rng;
// use salesman::*;

// #[test]
// fn test_20_points() {
//     let mut points = Vec::new();
//     for _ in 0..20 {
//         let point = Point {
//             x: rand::thread_rng().gen_range(-1.0..1.0),
//             y: rand::thread_rng().gen_range(-1.0..1.0),
//         };
//         points.push(point);
//     }
//     let result = salesman::solve(&points);
//     println!("{:?}", result);
// }
