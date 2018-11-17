#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    Integer(i64),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    pub object_type: ObjectType,
}

impl Object {
    pub fn inspect(&self) -> String {
        match self.object_type {
            ObjectType::Integer(x) => x.to_string(),
            ObjectType::Boolean(x) => x.to_string(),
            ObjectType::Null => format!("Null"),
        }
    }
}
