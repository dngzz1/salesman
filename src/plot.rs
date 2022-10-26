use crate::cluster;
use plotters::prelude::*;
pub fn plot_hamiltonian_loop(_points: &[(f32, f32)]) {
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

pub fn plot_clusters(_points: &[(f32, f32)], _salesmen_capacities: &[usize]) {
    // generate loop from points.
    let points = _points.to_vec();
    let salesmen_capacities = _salesmen_capacities.to_vec();
    let sorted_points = cluster::best_cluster(&points, &salesmen_capacities);

    // Plot the result and save to folder images/<filename>
    let my_colors = [&BLACK, &RED, &BLUE, &GREEN, &MAGENTA, &CYAN];

    let filename = "clustering";
    let pathname = &format!("images/{}.png", filename);
    let root_drawing_area = BitMapBackend::new(pathname, (768, 768)).into_drawing_area();
    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption("Clustering", ("sans-serif", 40))
        .build_cartesian_2d(-1.2_f32..1.2, -1.2_f32..1.2)
        .unwrap();

    for i in 0..salesmen_capacities.len() {
        let range_start = salesmen_capacities[0..i].iter().sum::<usize>();
        let range_end = salesmen_capacities[0..(i + 1)].iter().sum::<usize>();
        let range = range_start..range_end;
        ctx.draw_series(
            sorted_points[range]
                .iter()
                .map(|point| TriangleMarker::new(*point, 5, my_colors[i % my_colors.len()])),
        )
        .unwrap();
    }
}
