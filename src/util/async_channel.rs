///AsyncChannel AsyncChannel
pub struct AsyncChannel<T> {
    ///AsyncChannel tx
    pub tx: async_channel::Sender<T>,
    ///AsyncChannel rx
    pub rx: async_channel::Receiver<T>,
}
impl<T> AsyncChannel<T> {
    ///AsyncChannel new
    pub fn new(cap: usize) -> Self {
        let (tx, rx) = if cap > 0 {
            async_channel::bounded(cap)
        } else {
            async_channel::unbounded()
        };
        Self { tx, rx }
    }
}

impl<T> Clone for AsyncChannel<T> {
    ///AsyncChannel clone
    fn clone(&self) -> AsyncChannel<T> {
        Self {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}
