pub mod buffers;
pub mod math;
pub mod proxy;
pub mod raster;
pub mod vertex;

#[cfg(test)]
mod tests {
    #[test]
    fn test_proxy() {
        use crate::proxy::Proxy;

        struct IO {
            x: f32,
        }

        fn proxy_function(io: &mut IO) -> IO {
            IO { x: io.x + 2.0 }
        }

        let mut proxy = Proxy::new(proxy_function);

        let mut input = IO { x: 1.0 };

        assert_eq!((proxy.function)(&mut input).x, 3.0);
    }

    #[test]
    fn test_raster() {
        use crate::math::Point2D;
        use crate::raster::inside_triangle;

        let triangle = [
            Point2D { x: -0.5, y: 0.0 },
            Point2D { x: 0.0, y: 1.0 },
            Point2D { x: 0.5, y: 0.0 },
        ];

        assert_eq!(
            inside_triangle(&triangle, &Point2D { x: 0.0, y: 0.5 }),
            true
        );

        assert_eq!(
            inside_triangle(&triangle, &Point2D { x: 0.0, y: 2.0 }),
            false
        );
    }
}
