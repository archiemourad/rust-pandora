pub mod buffers;
pub mod raster;
pub mod shader;

#[cfg(test)]
mod tests {
    #[test]
    fn test_buffers() {
        use crate::buffers::{Buffer, Buffer2D};

        let mut buffer = Buffer::new(vec![1.0, 2.0, 3.0]);

        assert_eq!(buffer[0], 1.0);
        buffer[0] = 2.0;
        assert_eq!(buffer[0], 2.0);

        let mut buffer2d = Buffer2D::new(vec![vec![1.0, 2.0, 3.0]]);

        assert_eq!(buffer2d[(0, 0)], 1.0);
        buffer2d[(0, 0)] = 2.0;
        assert_eq!(buffer2d[(0, 0)], 2.0);
    }

    #[test]
    fn test_shader() {
        use crate::shader::Shader;

        struct IO {
            x: f32,
        }

        fn shader_function(io: &mut IO) -> IO {
            IO { x: io.x + 2.0 }
        }

        let mut shader = Shader::new(shader_function);

        let mut input = IO { x: 1.0 };

        assert_eq!((shader.function)(&mut input).x, 3.0);
    }

    #[test]
    fn test_raster() {
        use crate::raster::{edge_function, inside_triangle, Point2D};

        let line = [Point2D { x: 1.0, y: 0.0 }, Point2D { x: 1.0, y: 1.0 }];

        assert_eq!(edge_function(&line, &Point2D { x: 2.0, y: 0.5 }), true);
        assert_eq!(edge_function(&line, &Point2D { x: 0.0, y: 0.5 }), false);

        let triangle = [
            Point2D { x: 0.0, y: 0.0 },
            Point2D { x: 0.5, y: 1.0 },
            Point2D { x: 1.0, y: 0.0 },
        ];

        assert_eq!(
            inside_triangle(&triangle, &Point2D { x: 0.5, y: 0.5 }),
            true
        );
        assert_eq!(
            inside_triangle(&triangle, &Point2D { x: 0.0, y: 1.0 }),
            false
        );
    }
}
