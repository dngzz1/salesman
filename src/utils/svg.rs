use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

const COLORS: [&str; 12] = [
    "red",
    "green",
    "blue",
    "orange",
    "purple",
    "pink",
    "yellow",
    "brown",
    "plum",
    "teal",
    "turquoise",
    "maroon",
];

pub fn create_svg(stringing: &[Vec<(f32, f32)>]) -> Document {
    let mut document = Document::new().set("viewBox", (-1.2, -1.2, 2.4, 2.4));
    for i in 0..stringing.len() {
        let points = stringing.get(i).unwrap();
        let color = COLORS[i % 12];
        let path = create_path(points, color);
        let circles = create_circles(points, color);
        document = document.add(path).add(circles);
    }
    document
}

fn create_path(points: &[(f32, f32)], color: &str) -> Path {
    let mut path_data = Data::new();
    for (i, &point) in points.iter().enumerate() {
        if i == 0 {
            path_data = path_data.move_to(point);
        } else {
            path_data = path_data.line_to(point);
        }
    }
    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", 0.01)
        .set("d", path_data)
}

fn create_circles(points: &[(f32, f32)], color: &str) -> Group {
    let mut g = Group::new();
    for point in points {
        let circle = Circle::new()
            .set("cx", point.0)
            .set("cy", point.1)
            .set("fill", color)
            .set("r", 0.02);
        g = g.add(circle);
    }
    g
}
