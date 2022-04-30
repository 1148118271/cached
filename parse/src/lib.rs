pub mod set;
pub mod get;
pub mod remove;

#[derive(Debug)]
pub enum Type {
    Set,
    Get,
    Rm,
    Null
}

impl Type {
    pub fn get_type(text: &str) -> Type {
        let n: Vec<&str> = text.splitn(2, " ").collect();
        match *&n[0] {
            "set" => Type::Set,
            "get" => Type::Get,
            "rm" => Type::Rm,
            _ => Type::Null
        }
    }
}

pub trait Parse {
    fn new(text: &str) -> Result<Self, ()> where Self: Sized;
}
