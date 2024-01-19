use crate::math::Point3D;

/// A semi-generic vertex (struct) with (optional) `attributes` of type `A` and a `point` (position)
pub struct Vertex<A> {
    pub point: Point3D,
    pub attributes: Option<A>,
}

impl<A> Vertex<A> {
    pub fn new(point: Point3D, attributes: Option<A>) -> Self {
        Vertex { point, attributes }
    }
}
