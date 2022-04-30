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
        let r = regex::Regex::new(REGULAR).expect("regex new error.");
        if !r.is_match(text) {
            return Err(())
        }
        let r = regex::Regex::new(SPLIT_REGULAR).expect("regex new error.");
        let result: Vec<&str> = r.splitn(text, SIZE).collect();
        let key = *&result[1];
        Ok(GetParse {
            key: key.to_string(),
        })
    }
}