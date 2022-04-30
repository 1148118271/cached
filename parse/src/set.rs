use crate::Parse;

const REGULAR: &str = r"^(set)([\s]+)([\w]+)([\s]+)([\s\S]+)$";
const SPLIT_REGULAR: &str = r"[\s]+";
const SIZE: usize = 3;

#[derive(Debug)]
pub struct SetParse{
    pub key: String,
    pub value: String
}
impl Parse for SetParse {
    fn new(text: &str) -> Result<Self, ()> {
        let r = regex::Regex::new(REGULAR).expect("regex new error.");
        if !r.is_match(text) {
            return Err(())
        }
        let r = regex::Regex::new(SPLIT_REGULAR).expect("regex new error.");
        let result: Vec<&str> = r.splitn(text, SIZE).collect();
        let key = *&result[1];
        let value = *&result[2];
        Ok(SetParse {
            key: key.to_string(),
            value: value.to_string()
        })
    }
}