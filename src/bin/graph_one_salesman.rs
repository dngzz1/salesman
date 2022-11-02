use plotters::prelude::*;
fn main() {
    let seed = Some(42);
    let rand_points = salesman::utils::rand::get_points(20, seed);
    let distance_fn = salesman::utils::distance::euclidean;
    let distances = salesman::utils::distance::make_distance_vec(&rand_points, &distance_fn);
    let order =
        salesman::anneal::path::shortest_path_order(&rand_points, &distances, 2, true, seed);
    let points = salesman::anneal::path::get_path_from_order(&rand_points, &order);
    plot_hamiltonian_loop(&points);
}

fn plot_hamiltonian_loop(_points: &[(f32, f32)]) {
    // generate loop from points.
    let points = _points.to_vec();
    let salesman_loop;
    {
        let mut temp = points.clone();
        temp.push(temp[0]);
        salesman_loop = temp;
    }

    // Plot the result and save to folder images/<filename>
    let filename = "salesman";
    let pathname = &format!("images/{}.png", filename);
    let root_drawing_area = BitMapBackend::new(pathname, (768, 768)).into_drawing_area();
    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption("Travelling Salesman", ("sans-serif", 40))
        .build_cartesian_2d(-1.2_f32..1.2, -1.2_f32..1.2)
        .unwrap();

    ctx.draw_series(
        points
            .iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLACK)),
    )
    .unwrap();

    ctx.draw_series(
        vec![1]
            .iter()
            .map(|_| PathElement::new(salesman_loop.clone(), &BLUE)),
    )
    .unwrap();
}
