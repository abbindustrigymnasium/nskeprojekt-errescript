pub const PI: f64 = 3.14159265;

pub fn sine(mut x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }

    let mut n = 0;

    while x > PI {
        x -= PI;
        n += 1;
    }

    while x < 0.0 {
        x += PI;
        n += 1;
    }

    let mut factor = 1.0;
    if n % 2 == 1 {
        factor = -1.0;
    }

    if x > PI * 0.5 {
        x = PI - x;
    }

    return factor * (x - (x.powi(3) / 6.0) + (x.powi(5) / 120.0));
}

pub fn cosine(mut x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }

    if x < 0.0 {
        x *= -1.0;
    }

    let mut n = 0;

    while x > PI {
        x -= PI;
        n += 1;
    }

    let mut factor = 1.0;
    if n % 2 == 1 {
        factor = -1.0;
    }

    if x > PI * 0.5 {
        factor *= -1.0;
    }

    return factor * (1.0 - (x.powi(2) / 2.0) + (x.powi(4) / 24.0));
}
