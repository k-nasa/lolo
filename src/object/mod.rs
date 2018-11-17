enum ObjectType {
    Integer,
    Boolean,
}

trait Object {
    fn object_type() -> ObjectType;
    fn inspect(&self) -> String;
}

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn object_type() -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn object_type() -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}
