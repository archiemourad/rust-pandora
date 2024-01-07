/// A two-dimensional non-generic point (struct) with an `x` (x-coordinate) and `y` (y-coordinate)
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

/// A mathematical function that determines if a point (`point`) is to the left, right, or directly on a given line segment (`start` to `end`)
///
/// ### Arguments
///
/// * `start` - A reference to a `Point2D` (the start of the given line segment)
/// * `end` - A reference to a `Point2D` (the end of the given line segment)
/// * `point` - A reference to a `Point2D` (the given point to be evaluated)
///
pub fn edge_function(start: &Point2D, end: &Point2D, point: &Point2D) -> bool {
    (point.x - start.x) * (end.y - start.y) - (point.y - start.y) * (end.x - start.x) >= 0.0
}

/// A proxy-function for `edge_function` that determines if a point (`point`) is within a given triangle (`triangle`)
///
/// ### Arguments
///
/// * `triangle` - A reference to a 3-wide array of `Point2D`(s) (the given triangle)
/// * `point` - A reference to a `Point2D` (the given point to be evaluated)
///
pub fn inside_triangle(triangle: &[Point2D; 3], point: &Point2D) -> bool {
    for i in 0..3 {
        if !edge_function(&triangle[i], &triangle[(i + 1) % 3], point) {
            return false;
        }
    }

    true
}
