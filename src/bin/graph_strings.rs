use plotters::prelude::*;
use salesman::*;

fn main() {
    let num_points = 60;
    let seed = 42;
    let rand_points = salesman::example::rand_points_from_chacha(num_points, seed);
    let salesmen_capacities = [num_points / 6; 6];
    let points = salesman::cluster::best_cluster(&rand_points, &salesmen_capacities);
    plot(&points, &salesmen_capacities, true, "closed_strings");
    plot(&points, &salesmen_capacities, false, "open_strings");
}

fn plot(
    _sorted_points: &[(f32, f32)],
    _salesmen_capacities: &[usize],
    is_loop: bool,
    filename: &str,
) {
    // generate loop from points.
    let sorted_points = _sorted_points.to_vec();
    let salesmen_capacities = _salesmen_capacities.to_vec();

    // Plot the result and save to folder images/<filename>
    let my_colors = [&BLACK, &RED, &BLUE, &GREEN, &MAGENTA, &CYAN];

    let pathname = &format!("images/{}.png", filename);
    let root_drawing_area = BitMapBackend::new(pathname, (768, 768)).into_drawing_area();
    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption("Strings", ("sans-serif", 40))
        .build_cartesian_2d(-1.2_f32..1.2, -1.2_f32..1.2)
        .unwrap();

    for i in 0..salesmen_capacities.len() {
        let range_start = salesmen_capacities[0..i].iter().sum::<usize>();
        let range_end = salesmen_capacities[0..(i + 1)].iter().sum::<usize>();
        let range = range_start..range_end;
        let filtered_points = sorted_points[range].to_vec();
        let my_color = my_colors[i % my_colors.len()];

        // run salesman algorithm on filtered_points.
        let path_order = anneal::shortest_path_order(&filtered_points, 2, is_loop);
        let mut shortest_path = anneal::get_path_from_order(&filtered_points, &path_order);
        if is_loop {
            shortest_path.push(shortest_path[0]);
        }
        ctx.draw_series(
            filtered_points
                .iter()
                .map(|point| TriangleMarker::new(*point, 5, my_color)),
        )
        .unwrap();

        ctx.draw_series(
            vec![1]
                .iter()
                .map(|_| PathElement::new(shortest_path.clone(), my_color)),
        )
        .unwrap();
    }
}
