/// A two-dimensional non-generic point (struct) with an `x` (x-coordinate) and `y` (y-coordinate)
#[derive(Copy, Clone)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

/// A mathematical function that determines if a point (`point`) is to the left, right, or directly on a given line (`line`)
///
/// ### Arguments
///
/// * `line` - A reference to a 2-wide array of `Point2D`(s) (the given line)
/// * `point` - A reference to a `Point2D` (the given point)
///
pub fn edge_function(line: &[Point2D; 2], point: &Point2D) -> bool {
    (point.x - line[0].x) * (line[1].y - line[0].y)
        - (point.y - line[0].y) * (line[1].x - line[0].x)
        >= 0.0
}

/// A proxy-function for `edge_function` that determines if a point (`point`) is within a given triangle (`triangle`)
///
/// ### Arguments
///
/// * `triangle` - A reference to a 3-wide array of `Point2D`(s) (the given triangle)
/// * `point` - A reference to a `Point2D` (the given point)
///
pub fn inside_triangle(triangle: &[Point2D; 3], point: &Point2D) -> bool {
    let lines = [
        [triangle[0], triangle[1]],
        [triangle[1], triangle[2]],
        [triangle[2], triangle[0]],
    ];

    for line in lines {
        if !edge_function(&line, point) {
            return false;
        }
    }

    true
}
