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
mod tests {
    use super::*;
    #[test]
    fn test_intersects() {
        let a = (0.0, 0.0);
        let b = (1.0, 1.0);
        let c = (0.0, 1.0);
        let d = (1.0, 0.0);
        assert!(intersects(a, b, c, d));
        assert!(!intersects(a, d, b, c));
    }
}
