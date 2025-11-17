use serde::{Serialize, Deserialize};

trait RowType {
    fn to_string(&self) -> String;
    fn get_id(&self) -> String;
}

#[derive(Clone, Serialize, Deserialize)]
struct Person {
    id: String,
    name: String,
    age: u16,    
}


impl RowType for Person {
    fn to_string(&self) -> String {
        format!("name: {}, age: {}", self.name, self.age)
    }
    
    fn get_id(&self) -> String {
        self.id.clone()
    }

}


struct Collection<T: RowType> {
    items: Vec<T>,
    subscribers: Vec<Box<dyn Subscriber<T>>>,
}

impl<T: RowType> Collection<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            subscribers: Vec::new(),
        }
    }
    fn add(&mut self, item: T) {
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





trait Observable<T: RowType> {
    fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber<T>>);
    fn publish_add(&mut self, item: *const T);
}


trait Subscriber<T: RowType> {
    fn on_add(&mut self, item: &T);
}



struct Printer {
    
}

struct Filter<T: RowType> {
    predicate: Box<dyn Fn(&T) -> bool>,
    subscribers: Vec<Box<dyn Subscriber<T>>>,
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

struct Mapper<T: RowType> {
    transformer: Box<dyn Fn(&T) -> T>,
    subscribers: Vec<Box<dyn Subscriber<T>>>,
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

impl<T: RowType> Subscriber<T> for Printer {
    fn on_add(&mut self, item: &T) {
        println!("Added: {}", item.to_string());
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



fn main() {
    let mut people = Collection::new();
    people.add(Person {
            id: "1".to_string(),
            name: "John".to_string(),
            age: 30,
        });
    
    let mut mapper = Mapper {
        transformer: Box::new(|item: &Person| Person {
            id: item.id.clone(),
            name: item.name.clone(),
            age: item.age * 10,
        }),
        subscribers: vec![
            Box::new(Printer {}),
        ],
    };
    let filter = Filter {
        predicate: Box::new(|item: &Person| item.age > 30),
        subscribers: vec![
            Box::new(mapper),
        ],
    };
    people.add_subscriber(Box::new(filter));
    people.add(Person {
        id: "2".to_string(),
        name: "Jane".to_string(),
        age: 30,
    });
    people.add(Person {
        id: "2".to_string(),
        name: "Jane".to_string(),
        age: 31,
    });

    people.add(Person {
        id: "2".to_string(),
        name: "Jane".to_string(),
        age: 29,
    });

    people.add(Person {
        id: "2".to_string(),
        name: "Jane".to_string(),
        age: 31,
    });
    
}
