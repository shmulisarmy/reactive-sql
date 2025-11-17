use serde::{Serialize, Deserialize};
use crate::traits::RowType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub age: u16,    
}

impl RowType for Person {
    fn to_string(&self) -> String {
        format!("name: {}, age: {}", self.name, self.age)
    }
    
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
