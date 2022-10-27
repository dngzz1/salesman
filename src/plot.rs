use plotters::prelude::*;
pub fn plot_strings(
    _points: &[(f32, f32)],
    _order: &[usize],
    _salesmen_capacities: &[usize],
    is_loop: bool,
    display_string: bool,
    filename: &str,
    caption: &str,
) {
    let points = _points.to_vec();
    let order = _order.to_vec();
    let salesmen_capacities = _salesmen_capacities.to_vec();

    // Plot the result and save to folder images/<filename>
    let my_colors = [&BLACK, &RED, &BLUE, &GREEN, &MAGENTA, &CYAN];

    let pathname = &format!("images/{}.png", filename);
    let root_drawing_area = BitMapBackend::new(pathname, (768, 768)).into_drawing_area();
    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption(caption, ("sans-serif", 40))
        .build_cartesian_2d(-1.2_f32..1.2, -1.2_f32..1.2)
        .unwrap();

    for i in 0..salesmen_capacities.len() {
        let range_start = salesmen_capacities[0..i].iter().sum::<usize>();
        let range_end = salesmen_capacities[0..(i + 1)].iter().sum::<usize>();
        let range = range_start..range_end;
        let mut filtered_points = Vec::new();
        for i in range {
            filtered_points.push(points[order[i]]);
        }
        let my_color = my_colors[i % my_colors.len()];
        if is_loop {
            filtered_points.push(filtered_points[0]);
        }
        ctx.draw_series(
            filtered_points
                .iter()
                .map(|point| TriangleMarker::new(*point, 7, my_color)),
        )
        .unwrap();
        if display_string {
            ctx.draw_series(
                vec![1]
                    .iter()
                    .map(|_| PathElement::new(filtered_points.clone(), my_color)),
            )
            .unwrap();
        }
    }
}
