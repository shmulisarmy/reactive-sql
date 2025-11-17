use crate::traits::{Observable, RowType, Subscriber};

pub struct Collection<T: RowType> {
    pub items: Vec<T>,
    subscribers: Vec<Box<dyn Subscriber<T>>>,
}

impl<T: RowType> Collection<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            subscribers: Vec::new(),
        }
    }
    
    pub fn add(&mut self, item: T) {
        self.items.push(item);
        self.publish_add(self.items.last().unwrap());
    }
}

impl<T: RowType> Observable<T> for Collection<T> {
    fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber<T>>) {
        self.subscribers.push(subscriber);
    }

    fn publish_add(&mut self, item: *const T) {
        for subscriber in &mut self.subscribers {
            subscriber.on_add(unsafe { &*item });
        }
    }
}
