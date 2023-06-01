//!
//!
//!

use crate::la::Vector2;
use crate::la::X;
use crate::la::Y;

pub fn bresenham<F: FnMut(i32, i32)>(mut p0: Vector2<i32>, mut p1: Vector2<i32>, mut f: F) {
    let steep = (p1.x() - p0.x()).abs() < (p1.y() - p0.y()).abs();

    if steep {
        (*(&mut p0).x(), *(&mut p0).y()) = (p0.y(), p0.x());
        (*(&mut p1).x(), *(&mut p1).y()) = (p1.y(), p1.x());
    }

    if p0.x() > p1.x() {
        (*(&mut p0).x(), *(&mut p1).x()) = (p1.x(), p0.x());
        (*(&mut p0).y(), *(&mut p1).y()) = (p1.y(), p0.y());
    }

    let dx = p1.x() - p0.x();
    let dx2 = dx << 1;

    let slope = (p1.y() - p0.y()).abs() << 1;
    let mut slope_sum = 0;

    let step_y = if p0.y() > p1.y() { -1 } else { 1 };

    let mut x = p0.x();
    let mut y = p0.y();

    while x <= p1.x() {
        if steep {
            f(y, x);
        } else {
            f(x, y);
        }

        slope_sum += slope;

        if dx < slope_sum {
            y += step_y;
            slope_sum -= dx2;
        }

        x += 1;
    }
}
