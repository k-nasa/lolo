pub use self::ObjectType::*;

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
            Integer(x) => x.to_string(),
            Boolean(x) => x.to_string(),
            Null => "null".to_string(),
        }
    }

    // FIXME fron_int, from_boolは一般化できそうな雰囲気isある
    pub fn from_int(integer: i64) -> Object {
        Object {
            object_type: Integer(integer),
        }
    }

    pub fn from_bool(boolean: bool) -> Object {
        Object {
            object_type: Boolean(boolean),
        }
    }

    pub fn integer_value(&self) -> i64 {
        match self.object_type {
            Integer(x) => x,
            _ => panic!("faild: Object.integer_value"),
        }
    }

    pub fn boolean_value(&self) -> bool {
        match self.object_type {
            Boolean(x) => x,
            _ => panic!("faild: Object.boolean_value"),
        }
    }

    pub fn is_int(&self) -> bool {
        match self.object_type {
            Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self.object_type {
            Null => false,
            Boolean(false) => false,
            _ => true,
        }
    }
}

pub const TRUE: Object = Object {
    object_type: ObjectType::Boolean(true),
};
pub const FALSE: Object = Object {
    object_type: ObjectType::Boolean(false),
};
pub const NULL: Object = Object {
    object_type: ObjectType::Null,
};
