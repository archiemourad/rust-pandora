mod buffers;

#[cfg(test)]
mod tests {
    #[test]
    fn test_buffers() {
        use crate::buffers::{Buffer, Buffer2D};

        let mut buffer = Buffer::<f32> {
            data: vec![1.0, 2.0, 3.0],
        };

        assert_eq!(buffer[0], 1.0);
        buffer[0] = 2.0;
        assert_eq!(buffer[0], 2.0);

        let mut buffer2d = Buffer2D::<f32> {
            data: vec![vec![1.0, 2.0, 3.0]],
        };

        assert_eq!(buffer2d[(0, 0)], 1.0);
        buffer2d[(0, 0)] = 2.0;
        assert_eq!(buffer2d[(0, 0)], 2.0);
    }
}
