use std::boxed::Box;

/// A generic function-wrapper (struct) with an `I` (Input Type) and `O` (Output Type)
pub struct Wrapper<I, O> {
    pub function: Box<dyn FnMut(&mut I) -> O>,
}

impl<I, O> Wrapper<I, O> {
    pub fn new(function: impl FnMut(&mut I) -> O + 'static) -> Self {
        Wrapper {
            function: Box::new(function),
        }
    }
}
