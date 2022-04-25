use std::error::Error;

pub mod set;

#[derive(Debug)]
pub enum Type {
    Set,
    Get,
    Null
}

impl Type {
    pub fn get_type(text: &str) -> Type {
        let n: Vec<&str> = text.splitn(2, " ").collect();
        match *&n[0] {
            "set" => Type::Set,
            "get" => Type::Get,
            _ => Type::Null
        }

    }
}

pub trait Parse {
    fn new(text: &str) -> Result<Self, ()> where Self: Sized;
}
