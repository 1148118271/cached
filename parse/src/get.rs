use crate::Parse;


const REGULAR: &str = r"^(get)([\s]+)([\w]+)$";
const SPLIT_REGULAR: &str = r"[\s]+";
const SIZE: usize = 2;


#[derive(Debug)]
pub struct GetParse{
    pub key: String,
}

impl Parse for GetParse {
    fn new(text: &str) -> Result<Self, ()> {
        todo!()
    }
}