#[derive(Debug)]
pub enum EventMessage<E> {
    Event(E),
    Stop,
}
