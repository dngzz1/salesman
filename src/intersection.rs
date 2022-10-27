pub fn intersects(a: (f32, f32), b: (f32, f32), c: (f32, f32), d: (f32, f32)) -> bool {
    if a == c || a == d || b == c || b == d {
        return false;
    }
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
}

fn ccw(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> bool {
    (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
}

#[cfg(test)]
mod test_intersection;
