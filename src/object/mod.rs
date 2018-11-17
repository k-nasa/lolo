enum ObjectType {
    Integer,
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
}
