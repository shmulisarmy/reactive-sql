use crate::traits::{Observable, RowType, Subscriber};

pub struct Filter<T: RowType> {
    predicate: Box<dyn Fn(&T) -> bool>,
    subscribers: Vec<Box<dyn Subscriber<T>>>,
}

impl<T: RowType> Filter<T> {
    pub fn new(predicate: Box<dyn Fn(&T) -> bool>) -> Self {
        Self {
            predicate,
            subscribers: Vec::new(),
        }
    }
}

impl<T: RowType> Observable<T> for Filter<T> {
    fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber<T>>) {
        self.subscribers.push(subscriber);
    }

    fn publish_add(&mut self, item: *const T) {
        for subscriber in &mut self.subscribers {
            subscriber.on_add(unsafe { &*item });
        }
    }
}

impl<T: RowType> Subscriber<T> for Filter<T> {
    fn on_add(&mut self, item: &T) {
        if (self.predicate)(item) {
            for subscriber in &mut self.subscribers {
                subscriber.on_add(item);
            }   
        }
    }
}

pub struct Mapper<T: RowType> {
    transformer: Box<dyn Fn(&T) -> T>,
    subscribers: Vec<Box<dyn Subscriber<T>>>,
}

impl<T: RowType> Mapper<T> {
    pub fn new(transformer: Box<dyn Fn(&T) -> T>) -> Self {
        Self {
            transformer,
            subscribers: Vec::new(),
        }
    }
}

impl<T: RowType> Observable<T> for Mapper<T> {
    fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber<T>>) {
        self.subscribers.push(subscriber);
    }

    fn publish_add(&mut self, item: *const T) {
        for subscriber in &mut self.subscribers {
            subscriber.on_add(unsafe { &*item });
        }
    }
}

impl<T: RowType> Subscriber<T> for Mapper<T> {
    fn on_add(&mut self, item: &T) {
        let transformed = (self.transformer)(item);
        for subscriber in &mut self.subscribers {
            subscriber.on_add(&transformed);
        }
    }
}
