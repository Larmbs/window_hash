

#[allow(unused)]
pub trait DroppedValueConsumer<T> {
    fn consume( &self, value: T) {}
}

#[allow(unused)]
pub struct DefaultConsumer;
impl<T> DroppedValueConsumer<T> for DefaultConsumer {
    fn consume(&self, _value: T) {
        // Default implementation does nothing
    }
}
impl DefaultConsumer {
    pub fn new() -> Self {
        Self {}
    }
}