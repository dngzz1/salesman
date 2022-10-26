use plotters::prelude::*;

pub fn plot(_points: &[(f32, f32)]) {
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

pub fn plot_multi(_points: &[(f32, f32)], _salesmen_capacities: &[usize]) {
    // generate loop from points.
    let points = _points.to_vec();
    let salesmen_capacities = _salesmen_capacities.to_vec();

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

    for i in 0..salesmen_capacities.len() {
        let range_start = salesmen_capacities[0..i].iter().sum::<usize>();
        let range_end = salesmen_capacities[0..(i + 1)].iter().sum::<usize>();
        let range = range_start..range_end;
        let mut salesman_loop = points[range].to_vec();
        salesman_loop.push(salesman_loop[0]);
        ctx.draw_series(
            vec![1]
                .iter()
                .map(|_| PathElement::new(salesman_loop.clone(), &BLUE)),
        )
        .unwrap();
    }
}
