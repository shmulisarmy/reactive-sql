mod models;
mod traits;
mod collections;
mod operators;
mod subscribers;

use models::Person;
use collections::Collection;
use operators::{Filter, Mapper};
use subscribers::Printer;
use traits::Observable;

fn main() {
    let mut people = Collection::new();
    
    // Initial person
    people.add(Person {
        id: "1".to_string(),
        name: "John".to_string(),
        age: 30,
    });
    
    // Create a printer subscriber
    let printer = Printer;
    
    // Create a mapper that multiplies age by 10 and connect it to the printer
    let mut mapper = Mapper::new(Box::new(|item: &Person| Person {
        id: item.id.clone(),
        name: item.name.clone(),
        age: item.age * 10,
    }));
    mapper.add_subscriber(Box::new(printer));
    
    // Create a filter that only allows ages > 30 and connect it to the mapper
    let mut filter = Filter::new(Box::new(|item: &Person| item.age > 30));
    filter.add_subscriber(Box::new(mapper));
    
    // Add the filter as a subscriber to the collection
    people.add_subscriber(Box::new(filter));
    
    // Add some more people
    people.add(Person {
        id: "2".to_string(),
        name: "Jane".to_string(),
        age: 30,
    });
    
    people.add(Person {
        id: "3".to_string(),
        name: "Bob".to_string(),
        age: 31,
    });

    people.add(Person {
        id: "4".to_string(),
        name: "Alice".to_string(),
        age: 29,
    });

    people.add(Person {
        id: "5".to_string(),
        name: "Charlie".to_string(),
        age: 35,
    });
}
