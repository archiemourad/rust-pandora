use std::boxed::Box;

/// A generic function-proxy (struct) with an `IT` (InputType) and `OT` (OutputType)
pub struct Proxy<IT, OT> {
    pub function: Box<dyn FnMut(&mut IT) -> OT>,
}

impl<IT, OT> Proxy<IT, OT> {
    pub fn new(function: impl FnMut(&mut IT) -> OT + 'static) -> Self {
        Proxy {
            function: Box::new(function),
        }
    }
}
