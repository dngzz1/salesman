use plotters::prelude::*;
pub mod example;
use example::*;
use salesman::anneal::*;

fn main() {
    // Solve travelling salesman.
    let rand_block = rand_block(5, 6, 0.05, 0.08);
    let rand_points = rand_block.points;
    let points = shortest_path(&rand_points, 1);
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
