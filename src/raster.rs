use crate::math::Point2D;

/// A mathematical function that determines if a point (`point`) is to the left, right, or directly on a given line segment (`start` to `end`)
///
/// ### Arguments
///
/// * `start` - A reference to a `Point2D` (the start of the given line segment)
/// * `end` - A reference to a `Point2D` (the end of the given line segment)
/// * `point` - A reference to a `Point2D` (the given point to be evaluated)
///
/// ### Examples
///
/// ```
/// use pandora::math::Point2D;
/// use pandora::raster::edge_function;
///
/// // The line segment...
/// let start = Point2D { x: 0.0, y: 0.0 };
/// let end = Point2D { x: 0.0, y: 1.0 };
///
/// // The point to be evaluated...
/// let point = Point2D { x: 1.0, y: 0.5 };
///
/// // assert_eq! the point is to the right of the given line segment...
/// assert_eq!(edge_function(&start, &end, &point), true);
/// ```
///
pub fn edge_function(start: &Point2D, end: &Point2D, point: &Point2D) -> bool {
    (point.x - start.x) * (end.y - start.y) - (point.y - start.y) * (end.x - start.x) >= 0.0
}

const SEGMENTS: [(usize, usize); 3] = [(0, 1), (1, 2), (2, 0)];

/// A proxy-function for `edge_function` that determines if a point (`point`) is within a given triangle (`triangle`)
///
/// ### Arguments
///
/// * `triangle` - A reference to a 3-wide array of `Point2D`(s) (the given triangle)
/// * `point` - A reference to a `Point2D` (the given point to be evaluated)
///
/// ### Examples
///
/// ```
/// use pandora::math::Point2D;
/// use pandora::raster::inside_triangle;
///
/// // The triangle...
/// let triangle = [
///     Point2D { x: -0.5, y: 0.0 },
///     Point2D { x: 0.0, y: 1.0 },
///     Point2D { x: 0.5, y: 0.0 },
/// ];
///
/// // The point to be evaluated...
/// let point = Point2D { x: 0.0, y: 0.5 };
///
/// // assert_eq! the point is within the given triangle...
/// assert_eq!(inside_triangle(&triangle, &point), true);
/// ```
///
pub fn inside_triangle(triangle: &[Point2D; 3], point: &Point2D) -> bool {
    for segment in SEGMENTS {
        if !edge_function(&triangle[segment.0], &triangle[segment.1], point) {
            return false;
        }
    }

    true
}
