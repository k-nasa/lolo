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

    // FIXME fron_int, from_boolは一般化できそうな雰囲気isある
    pub fn from_int(integer: i64) -> Object {
        Object {
            object_type: ObjectType::Integer(integer),
        }
    }

    pub fn from_bool(boolean: bool) -> Object {
        Object {
            object_type: ObjectType::Boolean(boolean),
        }
    }

    pub fn integer_value(&self) -> i64 {
        match self.object_type {
            ObjectType::Integer(x) => x,
            _ => panic!("faild: Object.integer_value"),
        }
    }

    pub fn is_int(&self) -> bool {
        match self.object_type {
            ObjectType::Integer(_) => true,
            _ => false,
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
