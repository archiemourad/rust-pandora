use std::boxed::Box;

/// A generic function-proxy (struct) with an `I` (Input Type) and `O` (Output Type)
pub struct Proxy<I, O> {
    pub function: Box<dyn FnMut(&mut I) -> O>,
}

impl<I, O> Proxy<I, O> {
    pub fn new(function: impl FnMut(&mut I) -> O + 'static) -> Self {
        Proxy {
            function: Box::new(function),
        }
    }
}
