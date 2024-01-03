use std::boxed::Box;

/// A generic function-proxy (struct) with an `IT` (InputType) and `OT` (OutputType)
pub struct Shader<IT, OT> {
    pub function: Box<dyn FnMut(&mut IT) -> OT>,
}

impl<IT, OT> Shader<IT, OT> {
    pub fn new(function: impl FnMut(&mut IT) -> OT + 'static) -> Self {
        Shader {
            function: Box::new(function),
        }
    }
}
