use crate::Parse;

const REGULAR: &str = r"^(rm)([\s]+)([\w]+)$";
const SPLIT_REGULAR: &str = r"[\s]+";
const SIZE: usize = 2;


#[derive(Debug)]
pub struct RmParse{
    pub key: String,
}

impl Parse for RmParse {
    fn new(text: &str) -> Result<Self, ()> {
        let r = regex::Regex::new(REGULAR).expect("regex new error.");
        if !r.is_match(text) {
            return Err(())
        }
        let r = regex::Regex::new(SPLIT_REGULAR).expect("regex new error.");
        let result: Vec<&str> = r.splitn(text, SIZE).collect();
        let key = *&result[1];
        Ok(RmParse {
            key: key.to_string(),
        })
    }
}