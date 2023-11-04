pub struct HeapInvocable {
    data: Box<dyn FnMut() + Send + 'static>,
}

impl HeapInvocable {
    pub fn new<T>(data: T) -> HeapInvocable
    where
        T: FnMut() + Send + 'static,
    {
        return HeapInvocable {
            data: Box::new(data),
        };
    }

    pub fn invoke(&mut self) {
        (self.data)()
    }
}
