pub trait RowType: Clone + std::fmt::Debug {
    fn to_string(&self) -> String;
    fn get_id(&self) -> String;
}

pub trait Observable<T: RowType> {
    fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber<T>>);
    fn publish_add(&mut self, item: *const T);
}

pub trait Subscriber<T: RowType> {
    fn on_add(&mut self, item: &T);
}
