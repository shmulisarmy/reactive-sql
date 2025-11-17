use crate::traits::{RowType, Subscriber};

pub struct Printer;

impl<T: RowType> Subscriber<T> for Printer {
    fn on_add(&mut self, item: &T) {
        println!("Added: {}", item.to_string());
    }
}
