pub struct Cubic {
    i: usize,
    n: usize,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
}

impl Cubic {
    pub fn new(n: usize, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64,
           x3: f64, y3: f64) -> Cubic {

        Cubic {
            i: 0,
            n: n,
            x0: x0,
            y0: y0,
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            x3: x3,
            y3: y3,
        }
    }
}

impl Iterator for Cubic {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<(f64, f64)> {
        if self.i >= self.n {
            return None;
        }

        let t = self.i as f64 / ((self.n - 1) as f64);
        let t2 = t * t;
        let t3 = t2 * t;

        let ct = 1.0 - t;
        let ct2 = ct * ct;
        let ct3 = ct2 * ct;

        let x = ct3 * self.x0 +
                3.0 * ct2 * t * self.x1 +
                3.0 * ct * t2 * self.x2 +
                t3 * self.x3;

        let y = ct3 * self.y0 +
                3.0 * ct2 * t * self.y1 +
                3.0 * ct * t2 * self.y2 +
                t3 * self.y3;

        self.i += 1;

        Some((x, y))
    }
}
