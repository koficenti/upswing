// TODO! Replace Placeholder

pub struct Event<T> {
    callbacks: Vec<Box<dyn Fn(T) + 'static>>,
}

impl<T> Event<T> where T: Clone {
    pub fn new() -> Self {
        Self { callbacks: Vec::new() }
    }

    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(T) + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    pub fn trigger(&self, event: T) {
        for callback in &self.callbacks {
            callback(event.clone());
        }
    }
}