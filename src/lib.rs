pub mod buffers;
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
            x: f64,
        }

        fn shader_function(io: &mut IO) -> IO {
            IO { x: io.x }
        }

        let mut shader = Shader::new(shader_function);

        let mut input = IO { x: 1.0 };
        let output = (shader.function)(&mut input);

        assert_eq!(input.x, output.x);
    }
}
